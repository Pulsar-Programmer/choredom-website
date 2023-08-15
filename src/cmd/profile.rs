use crate::{db::{query, query_value}, AppData};
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
    query::<Account>(&mut db, q, Some((("rating", new_avg), ("review",review), ("username", username.as_str())))).await.unwrap()
    ;

    HttpResponse::Ok()
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SettingsData{
    username: String,
    displayname: String,
    location: String,
    bio: String,
    email: String,
    // pfp_pic: 
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SettingsData2{
    username1: String,
    username2: String,
    displayname: String,
    location: String,
    bio: String,
    email: String,
    // pfp_pic: 
}
impl SettingsData2{
    fn new(data: SettingsData, username2: String) -> Self{
        let SettingsData { username, displayname, location, bio, email } = data;
        Self { username1: username, username2, displayname, location, bio, email }
    }
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
    let mut settings_data = setting.into_inner();
    let username = retrieve_user(session).unwrap().unwrap();
    //edit stuff NOT together, as in, independently?

    let settings_data = SettingsData2::new(settings_data, username);

    let surrealql = "UPDATE accounts SET
        displayname = type::string($displayname),
        page.bio = type::string($bio),
        username = $username1,
        location = $location,
        email = $email
    WHERE username = type::string($username2);
    ";
    let mut db = data.db.lock().await;
    query_value(&mut db, surrealql, Some(settings_data)).await.unwrap();
    //might get a runtime error bcs of surrealql since password field is unused?

    HttpResponse::SeeOther().append_header((actix_web::http::header::LOCATION, "/settings")).body(SETTINGS)
}

#[get("/settings/upload")]
pub async fn upload() -> impl Responder{
    HttpResponse::Ok().body(UPLOAD)
}

#[post("/settings/upload-auth")]
pub async fn upload_auth(mut form: actix_multipart::Multipart, data: Data<AppData>, session: Session) -> Result<HttpResponse, actix_web::Error>{
    
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

    let new_state = super::signup::AccountState::Pending;
    let username = super::signup::retrieve_user(session).unwrap().unwrap();
    let params = (("state", "username"), (new_state, username));
    let surrealql = "UPDATE accounts SET state = $state WHERE username = $username;";
    
    let db = &mut data.db.lock().await;
    query_value(db, surrealql, Some(params)).await.unwrap();

    //review: is it really smooth or ok to have this return a Result?
    Ok(HttpResponse::SeeOther().append_header((actix_web::http::header::LOCATION, "/settings")).body(SETTINGS))
}




#[get("/settings/password")]
pub async fn password_change() -> impl Responder{
    // HttpResponse::Ok().body(UPLOAD)
    todo!() as HttpResponse
}

#[get("/settings/funds")]
pub async fn funds() -> impl Responder{
    // HttpResponse::Ok().body(UPLOAD)
    todo!() as HttpResponse
}

#[get("/settings/transfer-funds")]
pub async fn transfer_funds() -> impl Responder{
    // HttpResponse::Ok().body(UPLOAD)
    todo!() as HttpResponse
}

#[derive(serde::Serialize, serde::Deserialize)]
struct FundData{
    changed_funds: usize,
    password: String,
    //make an abstraction based on parts and forms and links in the js and buttons
    //add: bool,
}

#[post("/settings/funds/add")]
async fn deposit(form: Form<FundData>, data: web::Data<AppData>, session: Session) -> impl Responder{
    let FundData { changed_funds, password } = form.into_inner();
    let username = super::signup::retrieve_user(session).unwrap().unwrap();

    change_funds(true, &mut *data.db.lock().await, changed_funds).await.unwrap();
    HttpResponse::SeeOther().append_header((actix_web::http::header::LOCATION, "/settings")).body(SETTINGS)
}

#[post("/settings/funds/subtract")]
async fn spend(form: Form<FundData>, data: web::Data<AppData>, session: Session) -> impl Responder{
    let FundData { changed_funds, password } = form.into_inner();
    let username = super::signup::retrieve_user(session).unwrap().unwrap();
    change_funds(false, &mut *data.db.lock().await, changed_funds).await.unwrap();
    HttpResponse::SeeOther().append_header((actix_web::http::header::LOCATION, "/settings")).body(SETTINGS)
}

async fn change_funds(add: bool, db: &mut crate::db::Db, changed_funds: usize) -> anyhow::Result<()>{
    let surrealql = {
        if add{
            "UPDATE accounts SET balance += $balance WHERE username=$username;"
        }
        else{
            "UPDATE accounts SET balance -= $balance WHERE username=$username;"
        }
    };
    query_value(db, surrealql, Some(("balance", changed_funds))).await?;
    Ok(())
}








#[derive(serde::Serialize, serde::Deserialize)]
struct CreditsData{
    credits: usize,
    to_username: String,
    self_password: String,
    //make an abstraction based on parts and forms and links in the js and buttons
    //add: bool,
}


#[derive(serde::Serialize, serde::Deserialize)]
pub struct TransferData{
    credits: usize,
    to_username: String,
    self_username: String,
}


#[post("/settings/transfer-funds/transfer")]
async fn transfer(form: Form<CreditsData>, data: web::Data<AppData>, session: Session) -> impl Responder{
    
    let url = format!("https://www.paypal.com/sdk/js?client-id={}&currency=USD", 69696969);



    let CreditsData { credits, self_password, to_username } = form.into_inner();
    let self_username = super::signup::retrieve_user(session).unwrap().unwrap();
    let transferdata = TransferData{credits, self_username, to_username};

    let surrealql = "
    UPDATE accounts SET balance -= $credits WHERE username = $self_username;
    UPDATE accounts SET balance += $credits WHERE username = $to_username;
    ";
    query_value(&mut *data.db.lock().await, surrealql, Some(transferdata)).await.unwrap();


    HttpResponse::SeeOther().append_header((actix_web::http::header::LOCATION, "/settings")).body(SETTINGS)
}