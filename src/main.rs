use std::env::home_dir;

use actix_web::{get, post, web::{self, Form, Query}, App, HttpResponse, HttpServer, Responder};

macro_rules! website {
    ($($i:ident; $e:expr),+) => {
        $(
            pub const $i: &'static str = include_str!(concat!("../src-web/html/", $e, ".html"));
        )*
    };
}

macro_rules! wapp {
    ($($i:ident),+) => {
        App::new()
            .service(actix_files::Files::new("/src-web/static", "./src-web/static").show_files_listing())
            $(
                .service($i)
            )+
    };
}

website!(
    HOMEPAGE; "homepage", 
    CHAT; "chat", 
    LOGIN; "login",
    POST; "post",
    TASK; "task",
    SIGNUP; "signup",
    INDEX; "index",
    SIGNUPNEW; "signupnew"
);

// #[post("")]

#[get("/signupnew")]
async fn signupnew() -> impl Responder{
    HttpResponse::Ok().body(SIGNUPNEW)
}

#[get("/homepage")]
async fn homepage() -> impl Responder{
    HttpResponse::Ok().body(HOMEPAGE)
}

#[get("/index")]
async fn index() -> impl Responder{
    HttpResponse::Ok().body(INDEX)
}


#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    let p = format!("<p>Hello... {}</p>", name);
    HttpResponse::Ok().body(p)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        wapp!(greet, homepage, index, signupnew)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
#[derive(serde::Deserialize)]
struct FormData {
    username: String,
    password: String,
}
#[derive(serde::Deserialize, serde::Serialize)]
struct UserScore {
    userid: i64,
    name: String,
    rscore: i64,
    iscore: i64,
}