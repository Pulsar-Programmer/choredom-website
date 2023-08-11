use actix_web::cookie::SameSite;
use actix_web::{ web, App, HttpServer, cookie::Key};
// use actix_identity::IdentityMiddleware;
use actix_session::SessionMiddleware;
use actix_session_surrealdb::SurrealSessionStore;


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
    ($e:expr; $($i:ident),+) => {
        $e
            $(
                .service($i)
            )+
    };
}

// How to do Path extractor
// #[get("/hello/{name}")]
// async fn greet(name: web::Path<String>) -> impl Responder {
//     let p = format!("<p>Hello {}</p>", name);
//     HttpResponse::Ok().body(p)
// }

// How to do Identity login 
// #[get("/index")]
// async fn index(user: Option<Identity>) -> impl Responder {
//     if let Some(user) = user {
//         format!("Welcome! {}", user.id().unwrap())
//     } else {
//         "Welcome Anonymous!".to_owned()
//     }
// }

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let db = setup_db().await.unwrap();
    let app_state = web::Data::new(AppData {
        db: Arc::new(Mutex::new(db.clone())),
        transmitters: Arc::new(Transmitters::default())
    });
    // key needs to be generated outside the closure or else each worker gonna get a diff key
    let key = Key::generate();
    HttpServer::new(move|| {
        wapp!(
            App::new()
            .wrap(SessionMiddleware::builder(
                SurrealSessionStore::from_connection(db.clone(), "sessions"),
                key.clone(),
            ).cookie_same_site(SameSite::None).build())
            .service(actix_files::Files::new("/src-web/static", "./src-web/static").show_files_listing());
            homepage,
            signup, verify_email, home_redirect,
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
}
#[derive(Default)]
pub struct Transmitters{
    signup: Mutex<crate::cmd::signup::SignupTransmitter>,
    cct: Mutex<crate::cmd::chats::ChatClientTransmitter>,
}
pub trait Transmitter: Default{}