use actix_identity::IdentityMiddleware;
use actix_web::{web, App, HttpServer, cookie::Key, HttpResponse};
use actix_session::SessionMiddleware;
use actix_session_surrealdb::SurrealSessionStore;

mod cmd;
use chrono::Duration;
use cmd::homepage;
use cmd::signup::*;
use cmd::jobs::*;
use cmd::profile::*;
use cmd::chats::{chats_get, chats_obtain, receive, send, chat_nav, nav_links, pics_chats};
mod db;
use cmd::sites::NOUSER;
use db::setup_db;
mod img;

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
//Serve the test VVVVV
// pub const TEST: &str = include_str!(concat!("../src-web/html/", "test", ".html"));
// #[actix_web::get("/test")]
// pub async fn test() -> impl actix_web::Responder{
//     actix_web::HttpResponse::Ok().body(TEST)
// }


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    #[allow(clippy::expect_used)]
    let db = setup_db().await.expect("Database connection error.");
    let app_state = web::Data::new(AppData {
        db: Arc::new(Mutex::new(db.clone())),
    });

    // key needs to be generated outside the closure or else each worker gonna get a diff key
    let key = Key::generate();
    HttpServer::new(move|| {
        wapp!(
            App::new()
            .wrap(IdentityMiddleware::builder()
                .visit_deadline(#[allow(clippy::unwrap_used)] Some(Duration::days(30).to_std().unwrap()))
                .login_deadline(#[allow(clippy::unwrap_used)] Some(Duration::days(365).to_std().unwrap()))
                .build()
            )
            .wrap(SessionMiddleware::builder(
                SurrealSessionStore::from_connection(db.clone(), "sessions"),
                key.clone()
            ).build()).wrap(
                actix_web::middleware::ErrorHandlers::new()
                .handler(actix_web::http::StatusCode::NOT_FOUND, not_found)
            )
            .service(actix_files::Files::new("/temp", "./temp").show_files_listing())
            .service(actix_files::Files::new("/src-web/assets", "./src-web/assets").show_files_listing())
            .service(actix_files::Files::new("/src-web/static", "./src-web/static").show_files_listing());
            homepage,
            signup, verify_email, home_redirect_signup, 
            login, signin, signout, home_redirect_login,
            settings, settings_post,
            upload, upload_auth,
            post, post_job,
            tasks, tasks_in_area,
            profile, obtain_profile_data,
            rate,
            password_change, password_change_form,
            delete,
            dispute_management, contacts_form,
            transfer, transfer_funds,
            email_change, settings_email, home_redirect_settings,
            settings_present_data,
            jobs, jobs_data,
            chats_get, chats_obtain, send, receive,
            chat_nav, nav_links,
            delete_rating,
            pics_bio, pics_pfp, pics_chats //last time check of #143
            // ,test
        )
        .app_data(app_state.clone())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

use actix_web::{middleware::ErrorHandlerResponse, dev::ServiceResponse};
fn not_found<B>(res: ServiceResponse<B>) -> actix_web::error::Result<ErrorHandlerResponse<B>> {
    // split service response into request and response components
    let (req, res) = res.into_parts();
  
    // set body of response to modified body
    let res = res.set_body(NOUSER);
  
    // modified bodies need to be boxed and placed in the "right" slot
    let res = ServiceResponse::new(req, res)
        .map_into_boxed_body()
        .map_into_right_body();
  
    Ok(ErrorHandlerResponse::Response(res))
}

use std::sync::Arc;
use tokio::sync::Mutex;
use crate::db::Db;
pub struct AppData {
    pub db: Arc<Mutex<Db>>,
}
// impl AppData{
//     async fn obtain_db(&mut self) -> tokio::sync::MutexGuard<'_, Db>{
//         self.db.lock().await
//     }
// }
#[derive(serde::Serialize)]
pub struct RainError{
    message: String,
    for_user: bool,
}
impl RainError{
    // const function: fn(Self) = |x|{}; //< so weird

    pub fn from_message(message: impl ToString) -> Self{
        Self { message: message.to_string(), for_user: false }
    }
    pub fn for_user(mut self) -> Self{
        self.for_user = true;
        self
    }
    pub fn to_js(self) -> HttpResponse{
        HttpResponse::BadRequest().json(self)
    }
    pub fn for_js(message: impl ToString) -> HttpResponse {
        Self::from_message(message).to_js()
    }
    pub fn for_js_user(message: impl ToString) -> HttpResponse {
        Self::from_message(message).for_user().to_js()
    }
    pub fn for_html(message: impl ToString) -> HttpResponse{
        HttpResponse::BadRequest().body(message.to_string())
    }
    pub fn for_html_stderr() -> HttpResponse{
        HttpResponse::BadRequest().body(cmd::sites::ERRHTML)
    }
    pub fn for_see_other(message: impl ToString, header: &str) -> HttpResponse{
        HttpResponse::SeeOther().append_header((actix_web::http::header::LOCATION, header)).body(message.to_string())
    }
}
// impl From<RainError> for HttpResponse{
//     fn from(value: RainError) -> Self {
//         HttpResponse::BadRequest().json(value)
//     }
// }
// impl From<Box<dyn std::error::Error>> for RainError{
//     fn from(value: Box<dyn std::error::Error>) -> Self {
//         Self::from_message(value.to_string())
//     }
// }
//must we do some FromResidual stuff here? If I want to take any error, convert it into a RainError, and then propogate that as an HttpResponse, what is preventing me from doing so? It is basically an intermediate conversion between Box<dyn std::error::Error> -> HttpResponse.


// pub enum ResponderError{

// }

// struct ResponderResult{
//     inner: Result<Box<dyn Responder>, ResponderError>
// }


// pub type ResponderResult<T: Responder> = Result<T, ResponderError>;

// pub type ResponderResult = Result<impl Responder, ResponderError>;