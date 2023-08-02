// use crate::structs::{Account, Job, Money};
use actix_web::{get, post, Responder, web::{Data, Form, self}, HttpResponse};

use crate::{db::query, AppData};

use self::signup::Account;

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
    use actix_web::{get, post, Responder, web::{Data, Form, self}, HttpResponse, cookie::Cookie, HttpRequest};
    use super::{sites::*, signup::Account};
    use crate::{db::query, AppData};

    #[derive(serde::Deserialize)]
    pub struct LoginData{
        username: String,
        password: String,
    }

    #[get("/login")]
    pub async fn login() -> impl Responder{
        HttpResponse::Ok().body(LOGIN)
    }

    #[post("/signin")]
    pub async fn signin(form: Form<LoginData>, data : web::Data<AppData>) -> impl Responder{
        //Send email?
        let LoginData { username, password } = form.0;
        let mut db = data.db.lock().unwrap();
        let resp = login_cookie_response(HttpResponse::Ok().body(HOMEPAGE), &username);
        let result = query::<Account>(&mut db, "SELECT * FROM accounts WHERE username = type::string($username);", Some(("username", username))).await.unwrap();
        let result = result.get(0).unwrap().as_ref().unwrap();
        let len = result.len();
        if len > 1{
            todo!() // should never happen if correct things are true
        }
        else if len < 1{
            // ^feh 3
            return HttpResponse::Ok().body(SIGNUP)
        }
        let account = result.get(0).unwrap();
        if account.password != password{
            // ^feh 2
            HttpResponse::Ok().body(LOGIN)
        }
        else{
            resp
        }
    }


    pub fn login_cookie<'a>(username: &str) -> Cookie<'a>{
        // make sure you SANTIIZE THE USERNAME (what if it has special characters)
        // todo!()
        let value = format!("true;{username}");
        Cookie::build("login", value)
            .domain("localhost:8080")
            .path("/")
            .secure(true)
            .http_only(true)
            .finish()
    }

    pub fn login_cookie_response(mut resp: HttpResponse, username: &str) -> HttpResponse{
        if let Err(e) = resp.add_cookie(&login_cookie(username)){
            return HttpResponse::Ok().body(e.to_string())
        }
        resp
    }
}

mod settings{
    use actix_web::{get, post, Responder, web::{Data, Form}, HttpResponse};
    use super::sites::*;
    use crate::AppData;

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
async fn accounts(app_data: web::Data<AppData>) -> impl Responder{
    let mut db = app_data.db.lock().unwrap();
    let res2 = query::<Account>(&mut db, "SELECT * FROM accounts;", None::<()>).await.unwrap();
    let res1 = res2.get(0).unwrap();
    let result = res1.as_ref().unwrap();
    HttpResponse::Ok().body(format!("{result:?}"))
}

#[get("/")]
pub async fn homepage() -> impl Responder{
    HttpResponse::Ok().body(sites::HOMEPAGE)
}


// #[get("/")]
async fn tasks_in_area(app_data: web::Data<AppData>) -> impl Responder{

    let mut db = app_data.db.lock().unwrap();
    let res2 = query::<Account>(&mut db, "SELECT * FROM jobs;", None::<()>).await.unwrap();
    let res1 = res2.get(0).unwrap();
    let result = res1.as_ref().unwrap();
    HttpResponse::Ok()
}

struct Username{
    username: String,
}


async fn profile_data(app_data: web::Data<AppData>, username: Form<Username>) -> impl Responder{
    
    
    let mut db = app_data.db.lock().unwrap();
    let res2 = query::<Account>(&mut db, "SELECT * FROM accounts WHERE username = type::string($username);", Some(("username", username.0.username))).await.unwrap();
    let res1 = res2.get(0).unwrap();
    let result = res1.as_ref().unwrap();
    let len = result.len();
    if len > 1 {
        todo!() // should never happen
    }
    else if len < 1 {
        return HttpResponse::BadRequest()
    }

    HttpResponse::Ok()
}


#[get("/users/{username}")]
async fn profile(username: web::Path<String>) -> HttpResponse{
    //create a profile for username
    todo!()
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

