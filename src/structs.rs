use chrono as c;
use c::{DateTime, Utc};
use google_maps::distance_matrix::Location;
use google_maps::LatLng;


#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Account{
    display_name: String,
    username: String,
    creation_date: DateTime<Utc>,
    last_location: Location,

    email: String,
    data: AccountData,
    page: AccountPage,
    acct: AccountType,

    password: String,
    balance: Money,
}
impl Account{
    pub fn new(display_name: String, username: String, password: String, email: String) -> Self {
        Self { 
            display_name, 
            username, 
            creation_date: Utc::now(), 
            email, 
            data: AccountData::new(), 
            password, 
            balance: Money(0.), 
            page: AccountPage::new(),
            last_location: todo!(),
            acct: AccountType::Consumer,
        }
    }
}
#[derive(serde::Serialize, Debug, serde::Deserialize)]
enum AccountType{
    Consumer,
    Pending,
    Worker,
}




#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct AccountData{
    rating: f64,
    reviews: Vec<String>,
}
impl AccountData{
    fn new() -> Self{
        Self{ rating: 0., reviews: Vec::new() }
    }
}





#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct AccountPage{
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


pub type CmdResult<T> = Result<T, Box<dyn std::error::Error>>; //previously error handle
// pub type CmdResult<T> = Result<T, String>;



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
#[derive(serde::Deserialize)]
pub struct BasicAccount {
    pub email: String,
    pub password: String,
    pub password2: String,
    pub username: String,
    pub displayname: String,
}
pub struct AppState {
    pub code: Arc<Mutex<i64>>,
}


#[derive(serde::Deserialize)]
pub struct Code{
    pub code: i64
}










//ALL FROM BELOW: ADD THEIR RESPECTIVE LIBRARIES

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Money(pub f32);// use from another lib like color

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct Color(u8, u8, u8); //We need better info here







