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
        SETTINGS; "settings",
        PROFILE; "profile"
    );
}

pub mod signup;

pub mod jobs;

pub mod profile;

pub mod chats;

#[get("/")]
pub async fn homepage() -> impl Responder{
    HttpResponse::Ok().body(sites::HOMEPAGE)
}

// #[get("debug/{debug}")]
// pub async fn debug(debug: actix_web::web::Path<String>) -> impl Responder{
//     let ooga = include_str!(format!("../../src-web/html/{}.html", dbg));
//     HttpResponse::Ok().body(ooga)
// }

