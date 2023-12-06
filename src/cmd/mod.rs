use actix_web::{get, Responder, HttpResponse};

pub mod sites{
    #[macro_export]
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
        PROFILE; "profile",
        CONTACT; "contact",
        PASSWORD; "password",
        TRANSFER; "transfer",
        EMAIL_LOG; "email_login",
        EMAIL_CHANGE; "emailc",
        EMAIL_CHANGE_VERIFY; "emailc_ver",
        JOBS; "jobs",
        CHATNAV; "chat-nav",
        NOLOG; "nolog",
        NOUSER; "nouser"
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



