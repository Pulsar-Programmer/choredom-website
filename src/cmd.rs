// use crate::structs::{Account, Job, Money};
use actix_web::{get, post, HttpResponse};


pub mod sites{
    macro_rules! website {
        ($($i:ident; $e:expr),+) => {
            $(
                pub const $i: &'static str = include_str!(concat!("../src-web/html/", $e, ".html"));
            )*
        };
    }
    website!(
        HOMEPAGE; "homepage", 
        CHAT; "chat", 
        LOGIN; "login",
        POST; "post",
        TASK; "task",
        SIGNUP; "signup",
        EMAIL; "email",
        UPLOAD; "upload"
    );
}

pub mod signup{
    use super::sites::*;
    use crate::structs::{AppState, Transmitter, AccountState};
    use actix_web::{Responder, HttpResponse, get, web::{Form, self}, post};
    use rand::Rng;

    pub struct SignupTransmitter{
        pub state: AccountState,
        pub code: i64,
    }
    impl Transmitter for SignupTransmitter{}

    #[derive(serde::Deserialize)]
    pub struct SignupData {
        pub email: String,
        pub password: String,
        pub password2: String,
        pub username: String,
        pub displayname: String,
        // pub location: String,
    }

    #[derive(serde::Deserialize)]
    pub struct Code{
        pub code: i64
    }

    #[get("/signup")]
    pub async fn signup() -> impl Responder{
        HttpResponse::Ok().body(SIGNUP)
    }

    #[post("/verify-email")]
    pub async fn verify_email(app_data: web::Data<AppState>, form: Form<SignupData>) -> impl Responder{
        let SignupData { email: to_email, password, password2, username, displayname } = form.0;
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
        let mut code = app_data.transmitters.0.lock().unwrap();
        let codea = rand::thread_rng().gen_range(100000..1000000);
        (*code).code = codea;

        let email: Message = Message::builder()
            .from(from_email.parse().unwrap())
            .to(to_email.parse().unwrap())
            .subject("Welcome to Choredom")
            .body(format!("Welcome to Choredom, mf (my friend). Your verification code is {}", codea))
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
    pub async fn upload(app_data: web::Data<AppState>, code: Form<Code>) -> impl Responder{
        // println!("{} ; {}", code.0.code, *app_data.code.lock().unwrap());
        if code.0.code != app_data.transmitters.0.lock().unwrap().code{
            HttpResponse::Ok().body(EMAIL)
        }
        else{
            HttpResponse::Ok().body(UPLOAD)
        }
    }

    #[post("/upload-auth")]
    pub async fn upload_auth(mut form: actix_multipart::Multipart) -> Result<HttpResponse, actix_web::Error>{
        
        use futures::TryStreamExt;
        use futures::StreamExt;
        use std::io::Write;
        // iterate over multipart stream
        while let Ok(Some(mut field)) = form.try_next().await {
            let content_disposition = field.content_disposition();
            let filename = content_disposition.get_filename().unwrap();
            let filepath = format!("./tmp/{}", sanitize_filename::sanitize(&filename));
            let mut f = web::block(|| std::fs::File::create(filepath)).await.unwrap().unwrap();

            // Field in turn is stream of *Bytes* object
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                f = web::block(move || f.write_all(&data).map(|_| f)).await.unwrap()?;
            }
        }
        Ok(HttpResponse::Ok().into())
    }













}

pub mod login{
    use actix_web::{get, post, Responder, web::{Data, Form}, HttpResponse};
    use super::sites::*;

    #[derive(serde::Deserialize)]
    pub struct LoginData{
        email: String,
        password: String,
    }

    #[get("/login")]
    pub async fn login() -> impl Responder{
        HttpResponse::Ok().body(LOGIN)
    }

    #[post("/signin")]
    pub async fn signin(app_data: actix_web::web::Data<crate::structs::AppState>, form: Form<LoginData>) -> impl Responder{
        let dbpassword = String::new(); // get password from surreal.
        let dbemail = String::new(); // get email from surreal to confirm
        //if email exists in db then continue, else redirect to signup
        if dbpassword != form.password{
            HttpResponse::Ok().body(LOGIN)
        }
        else{
            *app_data.logged_in.lock().unwrap() = true;
            HttpResponse::Ok().body(HOMEPAGE)
        }
    }
}


#[actix_web::get("/task")]
pub async fn task() -> impl actix_web::Responder{
    HttpResponse::Ok().body(sites::TASK)
}

#[get("/accounts")]
pub async fn accounts() -> impl actix_web::Responder{
    serde_json::to_string(&1)
}






































pub fn register_job(title: String, body: String, location: String, time: String, price: String) -> Result<(), Box<dyn std::error::Error>>{
    use chrono::Utc;
    use chrono::TimeZone;
    //https://github.com/kelvins/US-Cities-Database
    // let (y, m, d) = ()
    let mut iter = time.split('-');
    let year = iter.next().ok_or("REGISTER JOB FN: Error parsing Date year.")?.parse()?;
    let month = iter.next().ok_or("REGISTER JOB FN: Error parsing Date month.")?.parse()?;
    let day = iter.next().ok_or("REGISTER JOB FN: Error parsing Date day.")?.parse()?;
    let time = Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).single().ok_or("REGISTER JOB FN: Invalid Date.")?;
    //time is written in the format: yyyy-mm-dd
    // let location = google_maps::distance_matrix::Location::LatLng(google_maps::LatLng::)

    let price = crate::structs::Money(price.parse()?);
    let job = crate::structs::Job::new(
        title,
        body,
        todo!(),
        time,
        price,
    );


    // Ok(())
    todo!("Function body isn't finished.");
}

