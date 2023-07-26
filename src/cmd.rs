use std::num::{ParseIntError, ParseFloatError};

use crate::structs::{Account, Job, Money, CmdResult};

pub fn register_user(display_name: String, username: String, password: String, email: String){
    let acct = Account::new(display_name, username, password, email);
    
}

pub fn register_job(title: String, body: String, location: String, time: String, price: String) -> CmdResult<()>{
    use chrono::Utc;
    use chrono::TimeZone;
    //https://github.com/kelvins/US-Cities-Database
    // let (y, m, d) = ()
    let mut iter = time.split('-');
    let year = iter.next().ok_or("REGISTER JOB FN: Error parsing Date year.")?.parse().map_err(|e: ParseIntError|e.to_string())?;
    let month = iter.next().ok_or("REGISTER JOB FN: Error parsing Date month.")?.parse().map_err(|e: ParseIntError|e.to_string())?;
    let day = iter.next().ok_or("REGISTER JOB FN: Error parsing Date day.")?.parse().map_err(|e: ParseIntError|e.to_string())?;
    let time = Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).single().ok_or("REGISTER JOB FN: Invalid Date.")?;
    //time is written in the format: yyyy-mm-dd
    // let location = google_maps::distance_matrix::Location::LatLng(google_maps::LatLng::)

    let price = crate::structs::Money(price.parse().map_err(|e: ParseFloatError|e.to_string())?);
    let job = crate::structs::Job::new(
        title,
        body,
        todo!(),
        time,
        price,
    );


    // Ok(())
    todo!("Function body isn't finished.");
}

