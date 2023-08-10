use crate::{db::{query, query_value, dissolve}, AppData};
use super::{signup::{Account, login_user, retrieve_user}, jobs::Job};
use super::sites::*;
use actix_session::Session;
use actix_web::{get, post, Responder, web::{Data, Form, self}, HttpResponse, HttpResponseBuilder};
struct Username{
    username: String,
}

async fn profile_data(app_data: web::Data<AppData>, username: Form<Username>) -> impl Responder{
    
    let mut db = app_data.db.lock().await;
    let res2 = query::<Account>(&mut db, "SELECT * FROM accounts WHERE username = type::string($username);", Some(("username", username.into_inner().username))).await.unwrap();
    let res1 = res2.get(0).unwrap();
    let result = res1.as_ref().unwrap();
    let len = result.len();
    if len != 1 {
        return HttpResponse::BadRequest() // should never happen
    }

    HttpResponse::Ok()
}


#[get("/users/{username}")]
async fn profile(username: web::Path<String>) -> HttpResponse{
    //create a profile for username
    todo!()
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct RatingData{
    stars: f32,
    body: String,
    username: String,
}

#[post("/rate/{username}")]
async fn rate(rating_data: Form<RatingData>, data: web::Data<AppData>, username: web::Path<String>) -> impl Responder{
    let review = rating_data.into_inner();
    let mut sum = review.stars.clamp(0., 5.);


    let mut db = data.db.lock().await;
    let res2 = query_value(&mut db, "SELECT page.reviews.stars FROM accounts WHERE username = type::string($username);", Some(("username", username.as_str()))).await.unwrap();
    let res1 = res2.get(0).unwrap();
    let result = res1.as_ref().unwrap();
    let len = result.len();
    if len != 1{
        //error but should not happen yada yada you know the drill
        return HttpResponse::BadRequest();
    }
    let res = result.get(0).unwrap();
    let stars = res.get("page").unwrap().get("reviews").unwrap().get("stars").unwrap().as_array().unwrap();
    // let mut sum = s;
    let div = stars.len() + 1;
    for i in stars{
        let j = i.as_f64().unwrap() as f32;
        sum += j;
    }
    let new_avg = sum / div as f32;

    let q = "UPDATE accounts
    SET 
    page.avg_rating = $rating,
    page.reviews += $review,
    WHERE username = type::string($username);";
    crate::db::dissolve(query::<Account>(&mut db, q, Some((("rating", "review", "username"), (new_avg, review, username.as_str())))).await, 34);

    HttpResponse::Ok()
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SettingsData{
    username: String,
    password: String,
    display_name: String,
    location: String,
    bio: String,
    // pfp_pic: 
}


#[get("/settings")]
pub async fn settings(app_data: Data<AppData>) -> impl Responder{
    // todo!();
    //get login data
    //give acct data
    HttpResponse::Ok().body(super::sites::SETTINGS)
}

#[post("/settings-post")]
pub async fn settings_post(session: Session, setting: Form<SettingsData>, data: Data<AppData>) -> impl Responder{
    // let accounts
    // let SettingsData { username, password: _, displayname, bio } = setting.0;
    //have a separate password and username changing mechanism 
    let username = retrieve_user(session).unwrap().unwrap().username;

    let surrealql = "UPDATE accounts SET 
        display_name = type::string($display_name),
        page.bio = type::string($bio)
    WHERE username = type::string($username);
    ";
    let mut db = data.db.lock().await;
    dissolve(query_value(&mut db, surrealql, Some(setting.into_inner())).await, 44);
    //might get a runtime error bcs of surrealql since password field is unused?

    HttpResponse::Ok()
}

#[get("/settings/upload")]
pub async fn upload() -> impl Responder{
    HttpResponse::Ok().body(UPLOAD)
}

#[post("/settings/upload-auth")]
pub async fn upload_auth(mut form: actix_multipart::Multipart) -> Result<HttpResponse, actix_web::Error>{
    
    use futures::TryStreamExt;
    use futures::StreamExt;
    use std::io::Write;
    // iterate over multipart stream
    while let Ok(Some(mut field)) = form.try_next().await {
        let content_disposition = field.content_disposition();
        let filename = content_disposition.get_filename().unwrap();
        let filepath = format!("./tmp/{}", sanitize_filename::sanitize(&filename));
        let mut f = web::block(|| std::fs::File::create(filepath)).await.unwrap().unwrap();

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f = web::block(move || f.write_all(&data).map(|_| f)).await.unwrap()?;
        }
    }
    Ok(HttpResponse::Ok().body(SETTINGS))
}