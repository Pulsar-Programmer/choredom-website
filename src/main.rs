use actix_web::{get, post, web::{self, Form, Query}, App, HttpResponse, HttpServer, Responder};
pub const INDEX: &'static str = include_str!("../src-web/html/index.html");
pub const CREATE: &'static str = include_str!("../src-web/html/create.html");
pub const COOKIE: &'static str = include_str!("../src-web/html/cookie.html");

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    let p = format!("<p>Hello... {}</p>", name);
    HttpResponse::Ok().body(p)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(actix_files::Files::new("/src-web/static", "./src-web/static").show_files_listing())
            .service(greet)
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