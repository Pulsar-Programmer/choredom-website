use actix_web::{ web, App, HttpServer, cookie::Key};
use actix_identity::IdentityMiddleware;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};

mod structs;
mod cmd;
use cmd::*;
use cmd::signup::*;
use cmd::profile::*;
use cmd::jobs::*;
// use cmd::settings::*;
// use cmd::
mod db;
use db::setup_db;

macro_rules! wapp {
    ($($i:ident),+) => {
        App::new()
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                Key::generate()
            ))
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
    let db = setup_db().await.unwrap();
    let app_state = web::Data::new(AppData {
        db: Arc::new(Mutex::new(db)),
        transmitters: Arc::new(Transmitters::default())
    });
    HttpServer::new(move|| {
        wapp!(
            homepage,
            signup, verify_email, settings_redirect,
            login, signin,
            settings, settings_post,
            upload, upload_auth,
            post, post_job,
            tasks, tasks_in_area
            // accounts
        )
        .app_data(app_state.clone())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

use std::sync::Arc;
use tokio::sync::Mutex;
use crate::db::Db;
pub struct AppData {
    pub db: Arc<Mutex<Db>>,
    pub transmitters: Arc<Transmitters> //add new transmitters as necessary and manually
} //nig
#[derive(Default)]
pub struct Transmitters{
    signup: Mutex<crate::cmd::signup::SignupTransmitter>,
    cct: Mutex<crate::cmd::chats::ChatClientTransmitter>,
}
pub trait Transmitter: Default{}