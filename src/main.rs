use lettre::message::header::ContentType;
use lettre::{Message, Transport};
use lettre_email::EmailBuilder;
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
    EMAIL; "email"
);

#[post("/verify-email")]
async fn verify_email(form: Form<Account>) -> impl Responder{
    let Account { email, password, password2 } = form.0;
    println!("{email}, {password}, {password2}");
    if password != password2{
        return HttpResponse::Ok().body(SIGNUP);
    }
    // use lettre::message::header::ContentType;
    // use lettre::{Message, Transport};

    // let email = Message::builder()
    //     .from("NoBody <[email protected]>".parse().unwrap())
    //     .to("Hei <[email protected]>".parse().unwrap())
    //     .subject("Happy new year")
    //     .header(ContentType::TEXT_PLAIN)
    //     .body(String::from("Be happy!"))
    //     .unwrap();

    // use lettre::transport::smtp::authentication::Credentials;
    // use lettre::SmtpTransport;

    // let creds = Credentials::new("smtp_username".to_owned(), "smtp_password".to_owned());

    // let mailer = SmtpTransport::relay("smtp.gmail.com")
    //     .unwrap()
    //     .credentials(creds)
    //     .build();

    // match mailer.send(&email) {
    //     Ok(_) => println!("Email sent successfully!"),
    //     Err(e) => panic!("Could not send email: {:?}", e),
    // }


    HttpResponse::Ok().body(EMAIL)
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
    let p = format!("<p>Hello... {}</p>", name);
    HttpResponse::Ok().body(p)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        wapp!(greet, homepage, index, signupnew, verify_email)
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