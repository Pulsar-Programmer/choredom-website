use actix_web::body::BoxBody;
use chrono as c;
use c::{DateTime, Utc};
use google_maps::distance_matrix::Location;
use google_maps::LatLng;


#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Account{
    pub display_name: String,
    // pub username: String, USERNAME STORED IN DB AS ID
    pub creation_date: DateTime<Utc>,
    // pub last_location: Location,

    pub email: String,
    pub data: AccountData,
    pub page: AccountPage,
    pub state: AccountState,

    pub password: String,
    pub balance: Money,
}
impl Account{
    pub fn new(display_name: String, password: String, email: String) -> Self {
        Self { 
            display_name, 
            // username, 
            creation_date: Utc::now(), 
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

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Job{
    title: String,
    body: String,
    location: Location,
    time: DateTime<Utc>,
    price: Money,
}
impl Job{
    pub fn new(title: String, body: String, location: Location, time: DateTime<Utc>, price: Money) -> Job{
        Job { title, body, location, time, price }
    }
}



// #[derive(Error, Debug, Serialize)]
// pub struct CmdError {
//     #[serde(skip)]
//     source: anyhow::Error, // Source of the error
//     context: String, // Context information
//     // #[serde(skip)]
//     // #[backtrace]
//     // backtrace: Backtrace, //add backtracing eventually
// }
// impl Display for CmdError{
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }
use std::sync::{Arc, Mutex};
use crate::db::Db;
pub struct AppData {
    pub logged_in: Arc<Mutex<bool>>, //replace by a browser cookie?
    pub db: Arc<Mutex<Db>>,
    pub transmitters: Arc<(
        Mutex<crate::cmd::signup::SignupTransmitter>,
    )> //add new transmitters as necessary and manually
} //nig
pub trait Transmitter{}











#[derive(Debug)]
pub struct ActixSurrealError{
    inner: surrealdb::Error,
}
impl From<surrealdb::Error> for ActixSurrealError {
    fn from(error: surrealdb::Error) -> Self {
        ActixSurrealError { inner: error }
    }
}
impl actix_web::error::ResponseError for ActixSurrealError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        // return an appropriate status code based on the error
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::new(self.status_code())
        .set_body(BoxBody::new(self.inner.to_string()))
    }
}
impl std::fmt::Display for ActixSurrealError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}




//ALL FROM BELOW: ADD THEIR RESPECTIVE LIBRARIES

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Money(pub f32);// use from another lib like color

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct Color(u8, u8, u8); //We need better info here







