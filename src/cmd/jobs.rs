use crate::{db::{dissolve, query, query_value}, AppData};
use actix_web::{web::{Form, Data}, Responder, get, post, HttpResponse, HttpRequest, http::header::q};
use super::sites::{POST, TASK};
use chrono::{DateTime, Utc};

#[derive(serde::Deserialize)]
struct JobData{
    title: String,
    body: String,
    time: String, 
    price: f32,
}


#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Job{
    title: String,
    body: String,
    // location: Location, todo!()
    time: DateTime<Utc>,
    price: crate::structs::Money, 
    username: String,
}
impl Job{
    pub fn new(username: String, title: String, body: String, time: DateTime<Utc>, price: crate::structs::Money) -> Job{
        Job { username, title, body, time, price }
    }
}



#[get("/post-job")]
async fn post_job(form: Form<JobData>, req: HttpRequest, data: Data<AppData>) -> impl Responder{

    let cookie: String = req.cookie("login").map(|c|{c.value().to_string()}).unwrap_or("false;0".into());
    let (cookie, username) = cookie.split_once(';').unwrap_or(("false", "0"));
    let cookie = cookie.parse().unwrap_or(false);
    if !cookie{
        return HttpResponse::Forbidden().body("Cannot access contents; log in.")
    }
    
    let JobData { title, body, time, price } = form.0;

    use chrono::TimeZone;
    //https://github.com/kelvins/US-Cities-Database
    let mut iter = time.split('-');
    let year = iter.next().ok_or("REGISTER JOB FN: Error parsing Date year.").unwrap().parse().unwrap();
    let month = iter.next().ok_or("REGISTER JOB FN: Error parsing Date month.").unwrap().parse().unwrap();
    let day = iter.next().ok_or("REGISTER JOB FN: Error parsing Date day.").unwrap().parse().unwrap();
    let time = Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).single().ok_or("REGISTER JOB FN: Invalid Date.").unwrap();
    //time is written in the format: yyyy-mm-dd

    let job = Job::new(username.to_string(), title, body, time, crate::structs::Money(price));

    let mut db = data.db.lock().unwrap();
    dissolve(query_value(&mut db, "CREATE jobs SET data = $job", Some(("job", job))).await, 1);


    // todo!();
    HttpResponse::Ok().body(POST)
}

#[actix_web::get("/task")]
pub async fn task() -> impl Responder{
    todo!();
    HttpResponse::Ok().body(TASK)
}

#[get("/jobs/{id}")]
pub async fn jobs(id: actix_web::web::Path<String>, data: Data<AppData>,) -> impl Responder{
    //get job by id
    let mut db = data.db.lock().unwrap();
    let res2 = query::<Job>(&mut db, r#"SELECT type::thing("jobs", $id) FROM jobs;"#, Some(("id", id.as_str()))).await.unwrap();
    let res1 = res2.get(0).unwrap();
    let result = res1.as_ref().unwrap();
    let len = result.len();
    //there should never be more than one but just in case
    if len != 1 {
        //bad request 
    }
    let job = result.get(0).unwrap();
    //give job to frontend etc. etc. etc.
    todo!() as HttpResponse
}

// #[get("/")]
async fn tasks_in_area(app_data: Data<AppData>) -> impl Responder{


    let zipcode = String::new(); //get from database
    let mut db = app_data.db.lock().unwrap();
    let res2 = query::<Job>(&mut db, "SELECT * FROM jobs WHERE zipcode = string::new($zipcode);", Some(("zipcode", zipcode))).await.unwrap();
    let res1 = res2.get(0).unwrap();
    let result = res1.as_ref().unwrap();
    //give vector of jobs to frontend
    HttpResponse::Ok()
}