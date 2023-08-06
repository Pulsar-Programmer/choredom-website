use crate::AppData;
use crate::db::{dissolve, query_value};
use actix_web::{Responder, HttpResponse, web::{Form, self}, post};


#[derive(serde::Serialize, serde::Deserialize)]
struct FundData{
    changed_funds: f32,
}


#[post("/deposit")]
async fn deposit(form: Form<FundData>, data: web::Data<AppData>) -> impl Responder{
    
    let mut db = data.db.lock().await;
    let surrealql = "UPDATE accounts SET balance += $balance;";
    dissolve(query_value(&mut db, surrealql, Some(("balance", form.0.changed_funds))).await, 45);
    HttpResponse::Ok()
}

#[post("/spend")]
async fn spend(form: Form<FundData>, data: web::Data<AppData>) -> impl Responder{
    
    let mut db = data.db.lock().await;
    let surrealql = "UPDATE accounts SET balance -= $balance;";
    dissolve(query_value(&mut db, surrealql, Some(("balance", form.0.changed_funds))).await, 46);
    HttpResponse::Ok()
}


