use actix_identity::IdentityMiddleware;
use actix_web::{web, App, HttpServer, cookie::Key, HttpResponse};
use actix_session::SessionMiddleware;
use actix_session_surrealdb::SurrealSessionStore;

mod cmd;
use chrono::Duration;
use cmd::{homepage, policy, success};
use cmd::signup::*;
use cmd::jobs::*;
use cmd::profile::*;
use cmd::chats::{chats_get, chats_obtain, receive, send, chat_nav, nav_links, pics_chats, chats_access};
mod db;
use cmd::sites::NOUSER;
use db::setup_db;
use db::Db;
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
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug")); //logger
    let config = EnvConfig::from_env();
    let EnvConfig { web_addr, db_addr, web_port, .. } = config.clone();
    let db = setup_db(db_addr).await.expect("Database connection error.");
    let app_state = web::Data::new(AppData {
        db: db.clone(),
        config
    });

    // key needs to be generated outside the closure or else each worker gonna get a diff key
    let key = Key::generate();
    HttpServer::new(move|| {
        wapp!(
            App::new()
            .app_data(web::PayloadConfig::new(20 * 1024 * 1024)) // Set limit to 10MB
            .wrap(actix_web::middleware::Logger::default()) //logger
            .wrap(IdentityMiddleware::builder()
                .visit_deadline(#[allow(clippy::unwrap_used)] Some(Duration::days(30).to_std().unwrap()))
                .login_deadline(#[allow(clippy::unwrap_used)] Some(Duration::days(365).to_std().unwrap()))
                .build()
            )
            .wrap(SessionMiddleware::builder(
                SurrealSessionStore::from_connection(db.clone(), "sessions"),
                key.clone()
            ).build())
            .wrap(
                actix_web::middleware::ErrorHandlers::new()
                .handler(actix_web::http::StatusCode::NOT_FOUND, not_found)
            )
            .service(actix_files::Files::new("/usr/bio", "./tmp/bio"))
            .service(actix_files::Files::new("/usr/pfp", "./tmp/pfp"))
            // .service(actix_files::Files::new("/tmp/chats", "./tmp/chats").show_files_listing())
            .service(actix_files::Files::new("/src-web/assets", "./src-web/assets"))
            .service(actix_files::Files::new("/src-web/static", "./src-web/static"));
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
            pics_bio, pics_pfp, pics_chats,
            chats_access,
            policy, success, //last time check of #143
            my_jobs, my_jobs_get,
            delete_post, edit_post,
            set_theme, get_theme,
            report
            // , updates
            // ,test
        )
        .app_data(app_state.clone())
    })
    .bind((web_addr.as_str(), web_port))?
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

pub struct AppData {
    pub db: Db,
    pub config: EnvConfig,
}
#[derive(serde::Serialize)]
pub struct RainError{
    message: String,
    for_user: bool,
}
impl RainError{
    // const function: fn(Self) = |x|{}; //< so weird

    pub fn from_message(message: impl ToString) -> Self{
        let message = message.to_string();
        // println!("{message}");
        Self { message, for_user: false }
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


#[derive(Clone)]
pub struct EnvConfig{
    app_pwd: String,
    db_addr: String,
    web_addr: String,
    web_port: u16,
}
impl EnvConfig{
    fn from_env() -> Self{
        use std::env::var;
        dotenvy::dotenv().ok();

        Self {
            app_pwd: var("SMTP_PRIVATE_KEY").expect("Could not find SMTP_PRIVATE_KEY."),
            db_addr: var("DB_ADDR").expect("Missing DB_ADDR"),
            web_addr: var("WEB_ADDR").expect("Missing WEB_ADDR"),
            web_port: var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8080),
        }
    }
}



// pub enum ResponderError{

// }

// struct ResponderResult{
//     inner: Result<Box<dyn Responder>, ResponderError>
// }


// pub type ResponderResult<T: Responder> = Result<T, ResponderError>;

// pub type ResponderResult = Result<impl Responder, ResponderError>;
