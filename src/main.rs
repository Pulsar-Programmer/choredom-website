use actix_web::{get, post, web::{self, Form, Query}, App, HttpResponse, HttpServer, Responder};
use std::sync::{Arc, Mutex};
use structs::AppState;

mod structs;
mod cmd;
use cmd::sites::*;
use cmd::signup::*;
use cmd::login::*;
mod db;
use db::setup_db;

macro_rules! wapp {
    ($($i:ident),+) => {
        App::new()
            .service(actix_files::Files::new("/src-web/static", "./src-web/static").show_files_listing())
            $(
                .service($i)
            )+
    };
}

#[get("/")]
async fn homepage() -> impl Responder{
    HttpResponse::Ok().body(HOMEPAGE)
}

// #[get("/hello/{name}")]
// async fn greet(name: web::Path<String>) -> impl Responder {
//     let p = format!("<p>Hello {}</p>", name);
//     HttpResponse::Ok().body(p)
// }

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let signuptransmitter = cmd::signup::SignupTransmitter{
        code: 0,
        state: structs::AccountState::Consumer
    };
    // let db = setup_db().await.unwrap();
    let app_state = web::Data::new(AppState {
        logged_in: Arc::new(Mutex::new(false)),
        // db: Arc::new(Mutex::new(db)),
        transmitters: Arc::new((
            Mutex::new(Box::new(signuptransmitter)),
        ))
    });
    let app_state = 5;
    HttpServer::new(move|| {
        wapp!(
            homepage,
            signup, verify_email, upload, upload_auth,
            login, signin
        )
        .app_data(app_state.clone())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}