use std::sync::atomic::AtomicUsize;

use crate::{db::{dissolve, query}, structs::AppData};
use actix_web::{web::{Form, self, Data}, Responder, get, post, HttpResponse, HttpRequest};
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

    let cookie: bool = req.cookie("login").map(|c|{c.value().parse().unwrap_or(false)}).unwrap_or(false);
    
    if !cookie{
        return HttpResponse::Forbidden().body("Cannot access contents; log in.")
    }
    
    let JobData { title, body, time, price } = form.0;
    let username = String::from("test"); // will be better later
    let job = Job::new(username, title, body, time, crate::structs::Money(price));

    let mut db = data.db.lock().unwrap();
    dissolve(query(&mut db, "CREATE jobs SET data = $job", ("job", job)).await, 1);


    // todo!();
    HttpResponse::Ok().body(POST)
}

#[actix_web::get("/task")]
pub async fn task() -> impl Responder{
    todo!();
    HttpResponse::Ok().body(TASK)
}