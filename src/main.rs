use actix_web::{get, post, web::{self, Form, Query}, App, HttpResponse, HttpServer, Responder};

mod structs;
mod cmd;
use cmd::sites::*;
use cmd::signup::*;
mod db;

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
    let app_state = web::Data::new(AppState {
        code: Arc::new(Mutex::new(0)),
    });
    HttpServer::new(move|| {
        wapp!(
            homepage,
            signup, verify_email, upload, upload_auth
        )
        .app_data(app_state.clone())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}