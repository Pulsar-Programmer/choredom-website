use actix_web::body::BoxBody;
use chrono as c;
use c::{DateTime, Utc};
use google_maps::distance_matrix::Location;
use google_maps::LatLng;



use std::sync::{Arc, Mutex};
use crate::db::Db;
pub struct AppData {
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







