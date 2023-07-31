use actix_web::body::BoxBody;
use chrono as c;
use c::{DateTime, Utc};
use google_maps::distance_matrix::Location;
use google_maps::LatLng;





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











//ALL FROM BELOW: ADD THEIR RESPECTIVE LIBRARIES

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Money(pub f32);// use from another lib like color

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct Color(u8, u8, u8); //We need better info here







