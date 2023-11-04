use actix_identity::IdentityMiddleware;
use actix_web::{ web, App, HttpServer, cookie::Key};
use actix_session::SessionMiddleware;
use actix_session_surrealdb::SurrealSessionStore;

mod cmd;
use chrono::Duration;
use cmd::*;
use cmd::signup::*;
use cmd::profile::*;
use cmd::jobs::*;
// use cmd::chats::chat::{chat, chat_ws};
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
    });

    // key needs to be generated outside the closure or else each worker gonna get a diff key
    let key = Key::generate();
    HttpServer::new(move|| {
        wapp!(
            App::new()
            .wrap(IdentityMiddleware::builder()
                .visit_deadline(Some(Duration::days(30).to_std().unwrap()))
                .login_deadline(Some(Duration::days(365).to_std().unwrap()))
                .build()
            )
            .wrap(SessionMiddleware::builder(
                SurrealSessionStore::from_connection(db.clone(), "sessions"),
                key.clone()
            ).build())
            .service(actix_files::Files::new("/src-web/static", "./src-web/static").show_files_listing());
            homepage,
            signup, verify_email, home_redirect_signup, home_redirect_login,
            login, signin, signout,
            settings, settings_post,
            upload, upload_auth,
            post, post_job,
            tasks, tasks_in_area,
            profile, rate,
            password_change, password_change_form,
            delete,
            dispute_management, contacts_form,
            transfer, transfer_funds,
            email_change, settings_email, home_redirect_settings,
            settings_present_data
            // chat, chat_ws
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
}