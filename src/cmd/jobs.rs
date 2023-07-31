use crate::{db::{dissolve, query, query_value}, AppData};
use actix_web::{web::{Form, Data}, Responder, get, post, HttpResponse, HttpRequest};
use super::sites::{POST, TASK};
use chrono::{DateTime, Utc};

#[derive(serde::Deserialize)]
struct JobData{
    title: String,
    body: String,
    time: DateTime<Utc>,
    price: f32, //later replace with money struct?
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