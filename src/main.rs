

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

#[post("/authenticate")]
async fn authenticate(form: Form<FormData>) -> impl Responder{
    let FormData { email, password, password2 } = form.0;
    println!("{email}, {password}, {password2}");
    if password != password2{

    }
    // Build a simple multipart message
    let message = mail_send::mail_builder::MessageBuilder::new()
        .from(("John Doe", "masonouni@gmail.com"))
        .to(vec![
            ("Jane Doe", "masonouni@gmail.com"),
            ("James Smith", "masonouni@gmail.com"),
        ])
        .subject("Hi!")
        .html_body("<h1>Hello, world!</h1>")
        .text_body("Hello world!");

    // Connect to the SMTP submissions port, upgrade to TLS and
    // authenticate using the provided credentials.
    mail_send::SmtpClientBuilder::new("smtp.gmail.com", 587)
        .implicit_tls(false)
        .credentials(("john", "p4ssw0rd"))
        .connect()
        .await
        .unwrap()
        .send(message)
        .await
        .unwrap();
    HttpResponse::Ok().body(SIGNUPNEW)
}

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
        wapp!(greet, homepage, index, signupnew, authenticate)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    password: String,
    password2: String,
}
#[derive(serde::Deserialize, serde::Serialize)]
struct UserScore {
    userid: i64,
    name: String,
    rscore: i64,
    iscore: i64,
}