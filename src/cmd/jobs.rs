use crate::{db::{query_once, query_once_option, sole_query}, AppData, cmd::sites::{NOLOG, NOUSER}, RainError, unwrap_identity};
use actix_identity::Identity;
use actix_web::{web::{Data, self, Json}, Responder, get, post, HttpResponse};
use serde::Serialize;
use surrealdb::sql::Thing;
use super::{sites::{POST, TASK}, signup::AccountPage};
use chrono::{DateTime, Utc, TimeZone};
use super::signup::AccountState;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct JobData{
    title: String,
    body: String,
    time: String, 
    price: String,
    location: String,
}
impl JobData{

    fn to_job(self) -> Result<Job, HttpResponse>{
        let Self { title, body, time, price, location } = self;

        if title.trim().is_empty(){
            return Err(RainError::for_js_user("Title is empty!"));
        }

        if location.is_empty(){
            return Err(RainError::for_js_user("Please enter a location for the job."))
        }

        //https://github.com/kelvins/US-Cities-Database
        //its all cool if the location doesn't exist; people just won't see the job ¯\_(ツ)_/¯
        let mut iter = time.split('-');
        let (Some(year), Some(month), Some(day)) = (iter.next(), iter.next(), iter.next())  else { return Err(RainError::for_js_user("Ensure to enter a date!"))};
        let (Ok(year), Ok(month), Ok(day)) = (year.parse(), month.parse(), day.parse()) else { return Err(RainError::for_js_user("Ensure to enter a valid date!"))};
        let Some(time) = Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).single() else { return Err(RainError::for_js_user("Ensure to enter a valid date!"))};
        //time is written in the format: yyyy-mm-dd
        let Ok(price) = price.parse::<f32>() else { return Err(RainError::for_js_user("The price could not be resolved."))};

        if price.is_nan() || price.is_infinite() || price <= 0. {
            return Err(RainError::for_js_user("Enter a valid price!"))
        }

        let job = Job::new(title, body, time, (price * 100.0) as u64, location);
        Ok(job)
    }
}


#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Job{
    title: String,
    body: String,
    time: DateTime<Utc>,
    price: u64, 
    location: String,
}
impl Job{
    pub fn new(title: String, body: String, time: DateTime<Utc>, price: u64, location: String) -> Job{
        Job { title, body, time, price, location}
    }
}

#[get("/post-job")]
pub async fn post(identity: Option<Identity>) -> impl Responder{
    if identity.is_none(){
        return HttpResponse::Ok().body(NOLOG);
    }
    HttpResponse::Ok().body(POST)
}

#[post("/post-job-2")]
pub async fn post_job(form: web::Json<JobData>, data: Data<AppData>, identity: Option<Identity>) -> impl Responder{
    // let user = request.get_identity();
    // let username = user.unwrap().id().unwrap();
    let Ok(username) = super::signup::unwrap_identity(identity) else { return RainError::for_js("Illegal identity travel.")};
    
    let job = match form.into_inner().to_job(){
        Ok(j) => j,
        Err(e) => return e,
    };

    let surrealql = 
    r#"
    BEGIN TRANSACTION;
        LET $id = (SELECT id FROM accounts WHERE username=type::string($username))[0].id;
        CREATE jobs SET data = $job, user = type::thing("accounts", $id);
    COMMIT TRANSACTION;"#;
    //^feh PLEASE MAKE SURE TO ERROR HANDLE WHAT HAPPENS IF THERE ARE NO ACCOUNTS WITH THAT USERNA<E
    let mut db = data.db.lock().await;

    //check for verification
    let Ok(Some(a)) = query_once_option::<super::signup::AccountState>(&mut db, "SELECT * FROM (SELECT state FROM accounts WHERE username=$username).state;", ("username", &username)).await else { return RainError::for_html(NOUSER)};
    match a {
        super::signup::AccountState::Verified => {},
        _ => {return RainError::for_js_user("You must be verified to post a job!")}
    }

    if let Err(e) = sole_query(&mut db, surrealql, JobUsername{ job, username }).await { return RainError::for_js(e) };

    // HttpResponse::SeeOther().append_header((actix_web::http::header::LOCATION, "/post-job")).body(POST)
    HttpResponse::Ok().finish()
}

#[derive(serde::Serialize)]
struct JobUsername{
    job: Job,
    username: String,
}



#[get("/jobs/{id}")]
pub async fn jobs(jobid: actix_web::web::Path<String>, data: Data<AppData>) -> impl Responder{
    let Ok(res1) = query_once::<JobPost>(&mut *data.db.lock().await, r#"SELECT * FROM jobs WHERE id=type::thing("jobs", $id) FETCH user.accounts;"#, ("id", jobid.into_inner())).await else { return RainError::for_html_stderr()};
    if res1.len() != 1{
        return HttpResponse::Ok().body(super::sites::NOUSER);
    }
    HttpResponse::Ok().body(super::sites::JOBS)
}

#[post("/jobs_attain")]
pub async fn jobs_data(data: Data<AppData>, path: web::Json<String>) -> impl Responder{
    let Ok(mut res1) = query_once::<JobPost>(&mut *data.db.lock().await, r#"SELECT * FROM jobs WHERE id=type::thing("jobs", $id) FETCH user.accounts;"#, ("id", path)).await else { return RainError::for_js("Error querying jobs.")};
    let Some(job) = res1.get_mut(0) else { return RainError::for_js("Jobs retreival error.") };
    let Ok(..) = job.timestamp_converted() else { return RainError::for_js("Timestamp conversion error.")};

    HttpResponse::Ok().content_type("application/json").json(job)
}



// #[derive(serde::Serialize, serde::Deserialize)]
// struct Address{
//     location: String,
// }

#[actix_web::get("/tasks")]
pub async fn tasks(identity: Option<Identity>) -> impl Responder{
    //Should we really lock access to this?
    if identity.is_none(){
        return HttpResponse::Ok().body(NOLOG);
    }
    HttpResponse::Ok().body(TASK)
}

struct FilterJobsJSON{
    location: String,
    similar_locations: bool,
    title_body: String,
    status: String,
    time: String,
    min_price: String,
    max_price: String,
}


// #[actix_web::route("/job-handling", method="GET", method="POST")]
#[actix_web::post("/job-handling")]
pub async fn tasks_in_area(app_data: Data<AppData>, js: web::Json<String>) -> impl Responder{
    // in the future allow filtering of multiple addresses.
    let address = js.into_inner();
    let Ok(res2) = query_once::<JobPost>(&mut *app_data.db.lock().await, "SELECT * FROM jobs WHERE data.location = type::string($location) FETCH user;", ("location", address)).await else { return RainError::for_js("Location query error.")};
    let result: Vec<_> = res2.into_iter().map(|mut a|{
        a.timestamp_converted().unwrap_or_default();
        a
    }).collect();
    HttpResponse::Ok().content_type("application/json").json(result)
}

struct FilterJobs{
    location: String,
    title_body: String,
    status: String,
    time: DateTime<Utc>,
    min_price: u32,
    max_price: u32,
}

fn tasks_build_query(filter: FilterJobsJSON) -> (String, FilterJobs){
    let mut str = String::new();
    let FilterJobsJSON{ location, title_body, status, time, min_price, max_price, similar_locations } = filter;
    // if similar_locations {
    //     str.push_str("WHERE data.location = $location");
    // }
    // else {
    //     str.push_str("WHERE data.location CONTAINS $location");
    // }
    str.push(';');
    let f = todo!();
    (str, f)
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct JobPost{
    id: Thing,
    
    data: JobPostData,

    user: JobRecordLink,
}
impl JobPost{
    fn timestamp_converted(&mut self) -> Result<(), Box<dyn std::error::Error>>{
        self.data.time = convert_timestamp(&self.data.time)?;
        // self.data.price /= 100.;
        Ok(())
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct JobPostData{
    title: String,
    body: String,
    time: String,
    price: u32, 
    location: String,
}



#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct JobRecordLink{
    displayname: String,
    username: String,
    state: AccountState,
    page: AccountPage,
}

//new model idea: have two types of functions, ones to call from js, and others to occur when you go to a certain page. They shouldn't have much overlap? IDK . WE CAN DO THISSSSSSSSSS

pub fn convert_timestamp(timestamp: &str) -> Result<String, Box<dyn std::error::Error>> {
    let datetime = DateTime::parse_from_rfc3339(timestamp)?.with_timezone(&Utc);
    Ok(datetime.format("%m/%d/%Y").to_string())
}

#[derive(serde::Deserialize)]
struct EditPostData{
    id: String,
    change: JobData,
}

#[derive(serde::Serialize)]
struct EditPostDataDB{
    id: String,
    change: Job,
    username: String,
}


#[post("/edit-post")]
pub async fn edit_post(identity: Option<Identity>, data: Data<AppData>, edit: Json<EditPostData>) -> impl Responder{
    let Ok(username) = unwrap_identity(identity) else { return RainError::for_js("Party island!")};
    
    let EditPostData { id, change } = edit.into_inner();
    //id should be internally defined
    //job_id should be given by the frontend
    //we must check that username matches the valid job_id

    let jobified_change = match change.to_job(){
        Ok(J) => J,
        Err(E) => return E,
    };

    let mut db = data.db.lock().await;
    let parameters = EditPostDataDB{ id, change: jobified_change, username };
    if let Err(e) = sole_query(&mut db, r#"UPDATE type::thing("jobs", $id) SET data = $change WHERE user.username = $username;"#, parameters).await { return RainError::for_js(e)};

    HttpResponse::Ok().finish()
}

#[derive(Serialize)]
struct IdUsername{
    id: String,
    username: String,
}


#[post("/delete-post")]
async fn delete_post(identity: Option<Identity>, data: Data<AppData>, job_id: Json<String>) -> impl Responder{
    let Ok(username) = unwrap_identity(identity) else { return RainError::for_js("Party island!")};

    let job_id = job_id.into_inner();
    let mut db = data.db.lock().await;
    
    let parameters = IdUsername{ id: job_id, username };
    if let Err(e) = sole_query(&mut db, r#"DELETE type::thing("jobs", $id) WHERE user.username=$username;"#, parameters).await { return RainError::for_js(e) };

    //job_id should be given by the frontend DONE
    //we must check that username matches the valid job_id DONE

    HttpResponse::Ok().finish()
}


#[get("my-jobs")]
async fn my_jobs(identity: Option<Identity>) -> impl Responder{
    if identity.is_none(){
        return RainError::for_html(NOLOG)
    }
    HttpResponse::Ok().body(crate::cmd::sites::MYJOBS)
}

#[post("myjobs-get")]
async fn my_jobs_get(identity: Option<Identity>, data: Data<AppData>) -> impl Responder{
    // println!("Called!");

    let Ok(username) = unwrap_identity(identity) else { return RainError::for_js("Cave island!")};

    let Ok(mut jobs_vec) = query_once::<JobPost>(&mut *data.db.lock().await, r#"SELECT * FROM jobs WHERE user.username = $username FETCH user;"#, ("username", username)).await else { return RainError::for_js("Error querying jobs.")};

    for job in jobs_vec.iter_mut(){
        if let Err(e) = job.timestamp_converted() { return RainError::for_js(e)};
    }

    // println!("Sent!");
    HttpResponse::Ok().content_type("application/json").json(jobs_vec)
}