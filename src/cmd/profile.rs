use crate::{db::{query, query_value}, AppData};
use super::{signup::Account, jobs::Job};
use actix_web::{get, post, Responder, web::{Data, Form, self}, HttpResponse};
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

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct RatingData{
    stars: f32,
    body: String,
}

#[get("/rate/{username}")]
async fn rate(rating_data: Form<RatingData>, data: web::Data<AppData>, username: web::Path<String>) -> impl Responder{
    let review = rating_data.0;
    let stars = review.stars.clamp(0., 5.);


    let mut db = data.db.lock().unwrap();
    let res2 = query_value(&mut db, "SELECT page.reviews.stars FROM accounts WHERE username = type::string($username);", Some(("username", username.as_str()))).await.unwrap();
    let res1 = res2.get(0).unwrap();
    let result = res1.as_ref().unwrap();
    

    let rating = todo!();

    let q = "UPDATE accounts
    SET 
    page.avg_rating = $rating,
    page.reviews += $review,
    WHERE username = type::string($username);";
    crate::db::dissolve(query::<Account>(&mut db, q, Some((("rating", "review", "username"), (rating, review, username.as_str())))).await, 34);


    




    todo!() as HttpResponse
    // HttpResponse::Ok().body()
}





#[get("/settings")]
async fn settings(app_data: Data<AppData>) -> impl Responder{
    todo!();
    //get login data
    //give acct data
    HttpResponse::Ok().body(super::sites::SETTINGS)
}

#[post("/settings-post")]
async fn settings_post() -> impl Responder{
    // let accounts
    todo!() as HttpResponse
}