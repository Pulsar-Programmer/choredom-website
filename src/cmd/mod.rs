// use crate::structs::{Account, Job, Money};
use actix_web::{get, post, Responder, web::{Data, Form}, HttpResponse};

pub mod sites{
    macro_rules! website {
        ($($i:ident; $e:expr),+) => {
            $(
                pub const $i: &'static str = include_str!(concat!("../../src-web/html/", $e, ".html"));
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

pub mod signup;

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

mod jobs;


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

