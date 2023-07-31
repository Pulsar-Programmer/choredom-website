use actix_web::{ web, App, HttpServer};
use std::sync::{ Arc, Mutex};

mod structs;
use structs::AppData;
mod cmd;
use cmd::*;
use cmd::sites::*;
use cmd::signup::*;
use cmd::login::*;
// use cmd::settings::*;
// use cmd::
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

// #[get("/hello/{name}")]
// async fn greet(name: web::Path<String>) -> impl Responder {
//     let p = format!("<p>Hello {}</p>", name);
//     HttpResponse::Ok().body(p)
// }

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let signuptransmitter = cmd::signup::SignupTransmitter{
        code: 0,
        state: cmd::signup::AccountState::Consumer
    };
    let db = setup_db().await.unwrap();
    let app_state = web::Data::new(AppData {
        db: Arc::new(Mutex::new(db)),
        transmitters: Arc::new((
            Mutex::new(signuptransmitter),
        ))
    });
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