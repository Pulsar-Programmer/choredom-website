// use crate::structs::{Account, Job, Money};
use actix_web::{get, Responder, HttpResponse};


// use {signup::Account, jobs::Job};

pub mod sites{
    #[macro_export]
    #[macro_use]
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
    use actix_web::{get, post, Responder, web::{Form, self}, HttpResponse, cookie::Cookie, HttpRequest};
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
        let mut db = data.db.lock().await;
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

pub mod jobs;

pub mod profile;

pub mod chats;

mod payment;

#[get("/")]
pub async fn homepage() -> impl Responder{
    HttpResponse::Ok().body(sites::HOMEPAGE)
}

// #[get("debug/{debug}")]
// pub async fn debug(debug: actix_web::web::Path<String>) -> impl Responder{
//     let ooga = include_str!(format!("../../src-web/html/{}.html", dbg));
//     HttpResponse::Ok().body(ooga)
// }

