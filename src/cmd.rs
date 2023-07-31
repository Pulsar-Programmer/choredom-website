// use crate::structs::{Account, Job, Money};
use actix_web::{get, post, Responder, web::{Data, Form}, HttpResponse};

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
        UPLOAD; "upload",
        SETTINGS; "settings"
    );
}

pub mod signup{
    use super::sites::*;
    use crate::structs::{AppData, Transmitter, Money};
    use crate::db::{dissolve, query};
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
        pub password2: String, //send through frontend differently
        pub username: String,
        pub displayname: String,
        // pub location: String,
    }

    #[derive(serde::Deserialize)]
    pub struct Code{
        pub code: i64
    }

    #[derive(serde::Deserialize, serde::Serialize, Debug)]
    pub struct Account{
        pub display_name: String,
        pub username: String, //USERNAME STORED IN DB AS ID
        pub creation_date: chrono::DateTime<chrono::Utc>,
        // pub last_location: Location,

        pub email: String,
        pub data: AccountData,
        pub page: AccountPage,
        pub state: AccountState,

        pub password: String,
        pub balance: Money,
    }
    impl Account{
        pub fn new(username: String, display_name: String, password: String, email: String) -> Self {
            Self { 
                display_name, 
                username, 
                creation_date: chrono::Utc::now(), 
                email, 
                data: AccountData::new(), 
                password, 
                balance: Money(0.), 
                page: AccountPage::new(),
                // last_location: todo!(),
                state: AccountState::Consumer,
            }
        }
    }
    #[derive(serde::Serialize, Debug, serde::Deserialize)]
    pub enum AccountState{
        Consumer,
        Pending,
        Worker,
    }

    #[derive(serde::Deserialize, serde::Serialize, Debug)]
    pub struct AccountData{
        rating: f64,
        reviews: Vec<String>,
    }
    impl AccountData{
        fn new() -> Self{
            Self{ rating: 0., reviews: Vec::new() }
        }
    }

    #[derive(serde::Deserialize, serde::Serialize, Debug)]
    pub struct AccountPage{
        pfp_url: String,
    }
    impl AccountPage{
        fn new() -> Self{
            Self{ 
                pfp_url: String::from("https://scontent-bos5-1.xx.fbcdn.net/v/t1.6435-9/95831445_10158064886431023_5042264117713305600_n.png?_nc_cat=111&ccb=1-7&_nc_sid=174925&_nc_ohc=jHdUksJywWcAX9BT5L0&_nc_ht=scontent-bos5-1.xx&oh=00_AfDnQ6lMQYJNm3VoLJiExu-JdGTp9T585V3NfmnukAornw&oe=64E0D75B"),  
            }
        }
    }

    #[get("/signup")]
    pub async fn signup() -> impl Responder{
        HttpResponse::Ok().body(SIGNUP)
    }

    #[post("/verify-email")]
    pub async fn verify_email(app_data: web::Data<AppData>, form: Form<SignupData>) -> impl Responder{
        let SignupData { email: to_email, password, password2, username, displayname } = form.0;
        // println!("{to_email}, {password}, {password2}");
        if password != password2{
            //better error handling
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
            .body(format!("Welcome to Choredom, {}. Your verification code is {}", displayname, codea))
            .unwrap();

        let creds: Credentials = Credentials::new(from_email.to_string(), smtp_key.to_string());

        // Open a remote connection to gmail
        let mailer: SmtpTransport = SmtpTransport::relay(&host)
            .unwrap()
            .credentials(creds)
            .build();

        // Send the email
        match mailer.send(&email) {
            Ok(_) => {println!("Email sent successfully!");},
            Err(e) => println!("{e}"), //handle this better later
        };
        
        let account: Account = Account::new(username, displayname , password, to_email);

        let mut db = app_data.db.lock().unwrap();
        // crate::db::register(&mut db, "accounts", username.as_str(), account).await;

        dissolve(query(&mut db, r#"
        CREATE type::thing("accounts", $username)
        SET
        display_name = type::string($display_name),
        creation_date = $creation_date,
        email = type::string($email),
        data = $data,
        page = $page,
        state = $state,
        password = type::string($password),
        balance = $balance;
        "#, account).await, 0);


        HttpResponse::Ok().body(EMAIL)
        
    }

    #[post("/upload")]
    pub async fn upload(app_data: web::Data<AppData>, code: Form<Code>) -> impl Responder{
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
    pub async fn signin(app_data: actix_web::web::Data<crate::structs::AppData>, form: Form<LoginData>) -> impl Responder{
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

mod settings{
    use actix_web::{get, post, Responder, web::{Data, Form}, HttpResponse};
    use super::sites::*;
    use crate::structs::AppData;

    #[get("/settings")]
    async fn settings(app_data: Data<AppData>) -> impl Responder{
        todo!();
        //get login data
        //give acct data
        HttpResponse::Ok().body(SETTINGS)
    }

    #[post("/settings-post")]
    async fn settings_post() -> impl Responder{
        // let accounts
        todo!();
        serde_json::to_string(&5)
    }

}

mod jobs{
    use crate::db::{dissolve, query};
    use actix_web::{web::{Form, self}, Responder, get, post, HttpResponse, HttpRequest};
    use super::sites::{POST, TASK};
    use chrono::{DateTime, Utc};

    #[derive(serde::Deserialize)]
    struct JobData{
        title: String,
        body: String,
        time: DateTime<Utc>,
        price: crate::structs::Money,
    }


    #[derive(serde::Deserialize, serde::Serialize, Debug)]
    pub struct Job{
        title: String,
        body: String,
        // location: Location, todo!()
        time: DateTime<Utc>,
        price: crate::structs::Money,
        username: String,
    }
    impl Job{
        pub fn new(username: String, title: String, body: String, time: DateTime<Utc>, price: crate::structs::Money) -> Job{
            Job { username, title, body, time, price }
        }
    }



    #[get("/post-job")]
    async fn post_job(form: Form<JobData>) -> impl Responder{


        todo!();
        HttpResponse::Ok().body(POST)
    }

    #[actix_web::get("/task")]
    pub async fn task() -> impl Responder{
        todo!();
        HttpResponse::Ok().body(TASK)
    }
}


#[get("/accounts")]
async fn accounts() -> impl Responder{
    serde_json::to_string(&1)
}

#[get("/")]
pub async fn homepage() -> impl Responder{
    HttpResponse::Ok().body(sites::HOMEPAGE)
}








































pub fn register_job(title: String, body: String, time: String, price: String) -> Result<(), Box<dyn std::error::Error>>{
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
    let job = jobs::Job::new(
        String::new(),
        title,
        body,
        time,
        price,
    );


    // Ok(())
    todo!("Function body isn't finished.");
}

