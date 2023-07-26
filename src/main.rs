mod structs;
mod cmd;
mod db;

use actix_web::{get, post, web::{self, Form, Query}, App, HttpResponse, HttpServer, Responder};
use rand::Rng;
use std::sync::{Arc, Mutex};

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
    EMAIL; "email",
    UPLOAD; "upload"
);

#[post("/verify-email")]
async fn verify_email(app_data: web::Data<AppState>, form: Form<Account>) -> impl Responder{
    let Account { email: to_email, password, password2 } = form.0;
    println!("{to_email}, {password}, {password2}");
    if password != password2{
        return HttpResponse::Ok().body(SIGNUP);
    }
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::{SmtpTransport, Transport};
    use lettre::Message;
    // let smtp_key: &str = "Brokies129gg";
    let smtp_key = "pjefpqhvsxmzomjf"; //app password
    let from_email: &str = "business@quannt.net";
    let host: &str = "smtp.gmail.com";
    let mut code = app_data.code.lock().unwrap();
    let codea = rand::thread_rng().gen_range(100000..1000000);
    *code = codea;

    let email: Message = Message::builder()
        .from(from_email.parse().unwrap())
        .to(to_email.parse().unwrap())
        .subject("Welcome to Choredom")
        .body(format!("Welcome to Choredom, mf (my friend). Your verification code is {}", code))
        .unwrap();

    let creds: Credentials = Credentials::new(from_email.to_string(), smtp_key.to_string());

    // Open a remote connection to gmail
    let mailer: SmtpTransport = SmtpTransport::relay(&host)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => {println!("Email sent successfully!"); HttpResponse::Ok().body(EMAIL)},
        Err(e) => HttpResponse::Ok().body(e.to_string()), //handle this better later
    }
    
}

#[post("/upload")]
async fn upload(app_data: web::Data<AppState>, code: Form<Code>) -> impl Responder{
    println!("{} ; {}", code.0.code, *app_data.code.lock().unwrap());
    if code.0.code != *app_data.code.lock().unwrap(){
        HttpResponse::Ok().body(EMAIL)
    }
    else{
        HttpResponse::Ok().body(UPLOAD)
    }
}

#[post("/upload-auth")]
async fn upload_auth(form: actix_multipart::Multipart) -> impl Responder{
    
}

#[get("/signup")]
async fn signupnew() -> impl Responder{
    HttpResponse::Ok().body(SIGNUP)
}

#[get("/")]
async fn homepage() -> impl Responder{
    HttpResponse::Ok().body(HOMEPAGE)
}

#[get("/index")]
async fn index() -> impl Responder{
    HttpResponse::Ok().body(INDEX)
}


#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    let p = format!("<p>Hello {}</p>", name);
    HttpResponse::Ok().body(p)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        code: Arc::new(Mutex::new(0)),
    });
    HttpServer::new(move|| {
        wapp!(greet, homepage, index, signupnew, verify_email, upload, upload_auth)
        .app_data(app_state.clone())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[derive(serde::Deserialize)]
struct Account {
    email: String,
    password: String,
    password2: String,
}
struct AppState {
    code: Arc<Mutex<i64>>,
}
#[derive(serde::Deserialize)]
struct Code{
    code: i64
}