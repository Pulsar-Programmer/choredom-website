use actix_identity::Identity;
use actix_web::{Responder, HttpResponse, web::Data, get, post};

use crate::AppData;


#[get("/shop")]
async fn shop(identity: Identity) -> impl Responder{
    todo!() as HttpResponse
}

#[post("/order")]
async fn buy(identity: Identity, data: Data<AppData>) -> impl Responder{
    todo!() as HttpResponse
}