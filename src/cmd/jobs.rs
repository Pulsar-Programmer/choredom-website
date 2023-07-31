use crate::db::{dissolve, query};
use actix_web::{web::{Form, self}, Responder, get, post, HttpResponse, HttpRequest};
use super::sites::{POST, TASK};
use chrono::{DateTime, Utc};

#[derive(serde::Deserialize)]
struct JobData{
    title: String,
    body: String,
    time: DateTime<Utc>,
    price: crate::structs::Money,
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
async fn post_job(form: Form<JobData>) -> impl Responder{

    


    todo!();
    HttpResponse::Ok().body(POST)
}

#[actix_web::get("/task")]
pub async fn task() -> impl Responder{
    todo!();
    HttpResponse::Ok().body(TASK)
}