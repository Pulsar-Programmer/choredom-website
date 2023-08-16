use crate::{db::{query, query_value}, AppData};
use super::signup::{Account, retrieve_user};
use super::sites::*;
use actix_session::Session;
use actix_web::{get, post, Responder, web::{Data, Form, self}, HttpResponse};


#[get("/users/{username}")]
pub async fn profile(username: web::Path<String>, app_data: Data<AppData>) -> impl Responder{

    let username = username.into_inner();
    
    let mut db = app_data.db.lock().await;
    let Ok(res2) = query::<Account>(&mut db, "SELECT * FROM accounts WHERE username = type::string($username);", Some(("username", username))).await else {
        return HttpResponse::BadRequest().finish();
    };
    let Some(res1) = res2.get(0) else {
        return HttpResponse::BadRequest().finish();
    };
    let Ok(result) = res1.as_ref() else{
        return HttpResponse::BadRequest().finish();
    };
    if result.len() != 1 {
        return HttpResponse::BadRequest().finish();
    }
    let Some(Account{username, displayname, creation_date, location: _, email: _, page, state, password:_, password_salt:_, balance:_}) = result.get(0) else{
        return HttpResponse::BadRequest().finish();
    };

    let mut html = format!(r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Profile Page</title>
    </head>
    <body>
        <div class="profile">
            <img href="{}"></img>
            <h1 id="displayName">Name: {}</h1>
            <h2 id="username">Username: {}</h2>
            <h3 id="AvgRating">Rating: {}</h3>
            <h4 id="CreationDate">Joined: {}</h4>
            <h5 id="State">{}</h5>
            <p id="bio">{}</p>
        </div><div class=ratings>
    "#, page.pfp_url, displayname, username, page.avg_rating, creation_date, state.as_str(), page.bio);
    for review in &page.reviews{
        html.push_str(&format!(
            r#"
            <div id="review">
            <h1 id="Rating">Rating: {}</h1>
            <h2>Poster Username: {}</h2>
            <p>{}</p>
            </div>
            "#, review.stars, review.username, review.body));
    }
    html.push_str("</div></body>");
    html.push_str(super::sites::PROFILE);
    HttpResponse::Ok().body(html)
}




#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct RatingData{
    stars: usize,
    body: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct PageRatingData{
    stars: usize,
    body: String,
    username: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct GroupRatingData{
    username: String,
    review: PageRatingData,
    new_avg: rust_decimal::Decimal,
}

#[post("/users/{username}/rate")]
pub async fn rate(rating_data: Form<RatingData>, data: web::Data<AppData>, username: web::Path<String>, session: Session) -> impl Responder{
    let RatingData { stars: sums, body } = rating_data.into_inner();
    let mut sum = sums.clamp(0, 5);
    let session_username = retrieve_user(session).unwrap().unwrap(); //make sure you cannot submit form if you are not signed in
    println!("{sum}, {body}");

    let mut db = data.db.lock().await;
    let res2 = query_value(&mut db, "SELECT page.reviews.stars FROM accounts WHERE username = type::string($username);", Some(("username", username.as_str()))).await.unwrap();
    let res1 = res2.get(0).unwrap();
    let result = res1.as_ref().unwrap();
    let len = result.len();
    if len != 1{
        return HttpResponse::BadRequest().finish();
    }
    let res = result.get(0).unwrap();
    let stars = res.get("page").unwrap().get("reviews").unwrap().get("stars").unwrap().as_array().unwrap();
    // let mut sum = s;
    let div = stars.len() + 1;
    for i in stars{
        let j = i.as_u64().unwrap() as usize;
        sum += j;
    }
    let new_avg = sum as f32 / div as f32;
    let new_avg = rust_decimal::Decimal::from_f32_retain(new_avg).unwrap();

    let q = "UPDATE accounts
    SET 
    page.avg_rating = $new_avg,
    page.reviews += $review
    WHERE username = type::string($username);";

    let review = PageRatingData{stars: sums, body, username: session_username};
    println!("{review:?}");

    query_value(&mut db, q, Some(GroupRatingData{username: username.into_inner(), review, new_avg})).await.unwrap();

    HttpResponse::SeeOther().append_header((actix_web::http::header::LOCATION, "/")).body(HOMEPAGE)
}













// -----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------














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















#[get("/contacts")]
async fn dispute_management() -> impl Responder{
    HttpResponse::Ok().body(crate::sites::CONTACT)
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ContactsInfo{

}

#[post("/contacts/form")]
async fn contacts_form(data: Data<AppData>, form: Form<ContactsInfo>) -> impl Responder{

    todo!() as HttpResponse
}