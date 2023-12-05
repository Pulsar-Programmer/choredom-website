use crate::{db::{query, query_value}, AppData, img::process_multipart};
use super::signup::{Account, retrieve_user, verify_password, email_user};
use super::sites::{TRANSFER, PASSWORD, SETTINGS, UPLOAD, HOMEPAGE, PROFILE, CONTACT, EMAIL_CHANGE_VERIFY, NOLOG};
use actix_identity::Identity;
use actix_multipart::Multipart;
use actix_session::Session;
use actix_web::{get, post, Responder, web::{Data, Form, self, Json}, HttpResponse};
use rust_decimal::prelude::ToPrimitive;

#[get("/users/{username}")]
pub async fn profile(_: web::Path<String>) -> impl Responder{
    HttpResponse::Ok().body(PROFILE)
}
#[derive(serde::Serialize)]
struct UsersFrontData<'a>{
    displayname: &'a String,
    pfp_url: &'a String,
    username: &'a String,
    avg_rating: f64,
    creation_date: String,
    state: &'a str,
    bio: &'a String,
    reviews: &'a Vec<PageRatingData>,
}

#[post("/obtain_profile")]
pub async fn obtain_profile_data(app_data: Data<AppData>, username: Json<String>) -> impl Responder{
    let username = username.into_inner();
    //^feh how do we handle what if the username is invalid, we must report this to the JS and not load the page or something
    let mut db = app_data.db.lock().await;
    let Ok(res2) = query::<Account>(&mut db, "SELECT * FROM accounts WHERE username = $username;", ("username", username)).await else {
        return HttpResponse::BadRequest().json("Issue with DB Queries.");
    };
    let Some(Ok(result)) = res2.get(0).as_ref() else {
        return HttpResponse::BadRequest().json("Issue with DB Query");
    };
    if result.len() != 1 {
        return HttpResponse::BadRequest().json("Account does not exist.");
    }
    let Some(Account{username, displayname, creation_date, location: _, email: _, page, state, password:_, password_salt:_, balance:_}) = result.get(0) else{
        return HttpResponse::BadRequest().finish();
    };

    let data = UsersFrontData{ 
        displayname, pfp_url: &page.pfp_url, username, avg_rating: page.avg_rating.to_f64().unwrap(), 
        creation_date: creation_date.format("%m/%d/%Y").to_string(), 
        state: state.as_str(), bio: &page.bio ,
        reviews: &page.reviews,
    };
    HttpResponse::Ok().json(data)
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
    rater: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct GroupRatingData{
    username: String,
    review: PageRatingData,
    new_avg: rust_decimal::Decimal,
}



#[post("/users/{username}/rate")]
pub async fn rate(rating_data: Json<RatingData>, data: web::Data<AppData>, username: web::Path<String>, identity: Option<Identity>) -> impl Responder{
    // println!("Queries");
    let rater: String = retrieve_user(identity.unwrap()).unwrap();
    let username = username.into_inner();
    if rater == username{
        //^feh
        return HttpResponse::BadRequest().json("You may not rate yourself!");
    }
    let mut db = data.db.lock().await;
    let chat_block = {
        let room_id = super::chats::RoomID::create([rater.clone(), username.clone()]);
        let res = query::<super::chats::ChatDBGiven>(&mut db, "SELECT messages[WHERE was_read=true] FROM chats WHERE room_id = $room_id;", ("room_id", &room_id)).await.unwrap();
        let result = res.get(0).unwrap().as_ref().unwrap();
        if result.len() != 1 {
            //^feh
            todo!("Error!")
        }
        let res = result.get(0).unwrap(); // ^ feh
        let mut not_contains_first = true;
        let mut not_contains_second = true;
        for i in &res.messages{
            if (rater > username) != i.sender{
                not_contains_first = false;
            }
            else{
                not_contains_second = false;
            }
            if !not_contains_first && !not_contains_second{
                break;
            }
        }
        not_contains_first || not_contains_second
    };
    if chat_block {
        return HttpResponse::BadRequest().body("You must work with the one you are rating in order to rate them!");
    }

    let RatingData { stars: sums, body } = rating_data.into_inner();
    let mut sum = sums.clamp(0, 5);

    let res2 = query::<Vec<PageRatingData>>(&mut db, "SELECT * FROM (SELECT page.reviews FROM accounts WHERE username = $username).page.reviews;", ("username", &username)).await.unwrap();
    //^^^^^ UPDATE THIS TO INCLUDE THE NEWLY SELECTED DATA
    let result = res2.get(0).unwrap().as_ref().unwrap();
    let len = result.len();
    if len != 1{
        return HttpResponse::BadRequest().finish();
    }
    let res = result.get(0).unwrap();

    let div = res.len() + if res.is_empty() {1} else{0};
    for PageRatingData{stars: star, rater: monkie, body: _} in res{
        if monkie==&rater
        {
            //^feh
            //this is also inefficient: use the Index feature and make a Rating table entirely to fix this entirely.
            return HttpResponse::BadRequest().body("You may not rate again! Delete your previous rating if you want to rate again!");
        }
        sum += star;
    }

    let new_avg = sum as f64 / div as f64;
    let new_avg = rust_decimal::Decimal::from_f64_retain(new_avg).unwrap();
    let q = "UPDATE accounts
    SET 
    page.avg_rating = $new_avg,
    page.reviews += $review
    WHERE username = $username;";

    let review = PageRatingData{stars: sums, body, rater};
    // println!("{review:?}");

    query_value(&mut db, q, GroupRatingData{username, review: review.clone(), new_avg}).await.unwrap();

    HttpResponse::Ok().json(review)
}

#[derive(serde::Serialize)]
struct DeleteRatingNote<'a>{
    new_avg: rust_decimal::Decimal,
    username: String,
    rater: &'a String,
}
#[derive(serde::Serialize)]
struct DeleteRatingFeedback{
    rater: String,
    new_avg: f64,
}

#[post("/users/{username}/rate/delete")]
pub async fn delete_rating(rater: Option<Identity>, username: web::Path<String>, data: Data<AppData>) -> impl Responder{
    let rater = retrieve_user(rater.unwrap()).unwrap();
    let username = username.into_inner();
    let mut db = data.db.lock().await;

    let mut sum = 0;
    let res2 = query::<Vec<PageRatingData>>(&mut db, "SELECT * FROM (SELECT page.reviews FROM accounts WHERE username = $username).page.reviews;", ("username", &username)).await.unwrap();
    //^^^^^ UPDATE THIS TO INCLUDE THE NEWLY SELECTED DATA
    let result = res2.get(0).unwrap().as_ref().unwrap();
    let len = result.len();
    if len != 1{
        return HttpResponse::BadRequest().finish();
    }
    let res = result.get(0).unwrap();
    
    let div = res.len() + if res.is_empty() {1} else{0};
    for PageRatingData{stars: star, rater: monkie, body: _} in res{
        if monkie==&rater
        {
            continue;
        }
        sum += star;
    }
    let new_avg_a = sum as f64 / div as f64;
    let new_avg = rust_decimal::Decimal::from_f64_retain(new_avg_a).unwrap();

    let query = "
    UPDATE accounts SET
    page.reviews -= (SELECT page.reviews FROM accounts WHERE username = $username AND page.reviews.rater CONTAINS $rater).page.reviews[0],
    page.avg_rating = $new_avg 
    WHERE username = $username;";

    //requires advanced DB query that can be done easily later
    query_value(&mut db, query, DeleteRatingNote{ new_avg, username, rater: &rater }).await.unwrap();

    HttpResponse::Ok().json(DeleteRatingFeedback{ rater, new_avg: new_avg_a })
}













// -----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------














#[derive(serde::Deserialize, serde::Serialize)]
pub struct SettingsData{
    username: String,
    displayname: String,
    location: String,
    bio: String,
    // pfp_pic: 
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SettingsData2{
    username1: String,
    username2: String,
    displayname: String,
    location: String,
    bio: String,
    // pfp_pic: 
}
impl SettingsData2{
    fn new(data: SettingsData, username2: String) -> Self{
        let SettingsData { username, displayname, location, bio } = data;
        Self { username1: username, username2, displayname, location, bio }
    }
}



#[get("/settings")]
pub async fn settings(identity: Option<Identity>) -> impl Responder{
    if identity.is_none(){
        return HttpResponse::Ok().body(NOLOG); 
    }
    HttpResponse::Ok().body(super::sites::SETTINGS)
}
#[derive(serde::Serialize, Debug)]
struct SettingsPresentData<'a>{
    username: &'a String,
    displayname: &'a String,
    location: &'a String,
    bio: &'a String,
}

#[post("/settings/present_data")]
pub async fn settings_present_data(app_data: Data<AppData>, identity: Option<Identity>) -> impl Responder{
    let mut db = app_data.db.lock().await;
    let q1 = query::<Account>(&mut db, "SELECT * FROM accounts WHERE username=$username;", ("username", retrieve_user(identity.unwrap()).unwrap())).await.unwrap();
    let curry_2 = q1.get(0).unwrap().as_ref().unwrap().get(0).unwrap();
    let Account { displayname, username, creation_date:_, location, email: _, page: super::signup::AccountPage { pfp_url:_, avg_rating:_, reviews:_, bio }, state:_, password:_, password_salt:_, balance:_ } = curry_2;
    let settings_data = SettingsPresentData{username, displayname, location, bio};
    //YESSS SO COOOLLL
    println!("{settings_data:?}");
    HttpResponse::Ok().content_type("application/json").json(settings_data)
}









#[post("/settings-post")]
pub async fn settings_post(identity: Option<Identity>, setting: Form<SettingsData>, data: Data<AppData>) -> impl Responder{
    let settings_data = setting.into_inner();
    let username = retrieve_user(identity.unwrap()).unwrap();
    //edit stuff NOT together, as in, independently?

    let settings_data = SettingsData2::new(settings_data, username);

    let surrealql = "UPDATE accounts SET
        displayname = $displayname,
        page.bio = $bio,
        username = $username1,
        location = $location
    WHERE username = $username2;
    ";
    let mut db = data.db.lock().await;
    query_value(&mut db, surrealql, settings_data).await.unwrap();
    //might get a runtime error bcs of surrealql since password field is unused?

    HttpResponse::SeeOther().append_header((actix_web::http::header::LOCATION, "/settings")).body(SETTINGS)
}

#[get("/settings/upload")]
pub async fn upload(identity: Option<Identity>) -> impl Responder{
    if identity.is_none(){
        return HttpResponse::Ok().body(NOLOG);
    }
    HttpResponse::Ok().body(UPLOAD)
}

#[post("/settings/upload/form")]
pub async fn upload_auth(form: actix_multipart::Multipart, data: Data<AppData>, identity: Option<Identity>) -> impl Responder{
    let username = super::signup::retrieve_user(identity.unwrap()).unwrap();
    let container = format!("verification/{username}");
    crate::img::process_multipart(form, container).await.unwrap();

    let new_state = super::signup::AccountState::PendingVerification;
    let params = (("state", "username"), (new_state, username));
    let surrealql = "UPDATE accounts SET state = $state WHERE username = $username;";
    
    let db = &mut data.db.lock().await;
    query_value(db, surrealql, params).await.unwrap();

    HttpResponse::SeeOther().append_header((actix_web::http::header::LOCATION, "/settings")).body(SETTINGS)
}




#[get("/settings/password")]
pub async fn password_change(identity: Option<Identity>) -> impl Responder{
    if identity.is_none(){
        return HttpResponse::Ok().body(NOLOG);
    }
    HttpResponse::Ok().body(PASSWORD)
}

#[derive(serde::Deserialize, serde::Serialize)]
struct PasswordData{
    p_old: String,
    p_new: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct PasswordChangeData{
    password: String,
    password_salt: String,
    username: String,
}

#[post("/settings/password/form")]
pub async fn password_change_form(data: Data<AppData>, form: Form<PasswordData>, identity: Option<Identity>) -> impl Responder{

    let PasswordData { p_old, p_new } = form.into_inner();

    let username = retrieve_user(identity.unwrap()).unwrap();

    let mut db = data.db.lock().await;
    let result = query::<Account>(&mut db, "SELECT * FROM accounts WHERE username = $username;", ("username", &username)).await.unwrap();
    let result = result.get(0).unwrap().as_ref().unwrap();
    if result.len() != 1{
        //^feh
        todo!() // should never happen if correct things are true
    }
    let Account { displayname: _, username: _, creation_date: _, location: _, email, page: _, state: _, password: p_old_2, password_salt: salt, balance: _ } = result.get(0).unwrap();

    email_user(email, "Your Choredom Password has been Changed", format!("Dear Choredom User,\n\tYour password has been changed from \n\t`{}`, \n\tto \n\t`{}`.", p_old, p_new)).unwrap();

    if !verify_password(&p_old, p_old_2, salt).unwrap() {
        //^feh incorrect passwords
        todo!()
    }

    let (password, password_salt) = super::signup::password_hash_argon2(p_new).unwrap();
    query_value(&mut db, "UPDATE accounts SET password = $password, password_salt = $password_salt WHERE username = $username", PasswordChangeData{password, password_salt: password_salt.to_string(), username}).await.unwrap();
    HttpResponse::SeeOther().append_header((actix_web::http::header::LOCATION, "/settings")).body(SETTINGS)
}







#[derive(serde::Deserialize)]
pub struct DeleteConfirmation{password:String}

#[post("/settings/delete")]
pub async fn delete(identity: Option<Identity>, password: Form<DeleteConfirmation>, data: Data<AppData>) -> impl Responder{
    let username = retrieve_user(identity.unwrap()).unwrap();
    let password_entered = password.into_inner().password;
    let mut db = data.db.lock().await;
    
    let result = query::<Account>(&mut db, "SELECT * FROM accounts WHERE username = $username;", ("username", &username)).await.unwrap();
    let result = result.get(0).unwrap().as_ref().unwrap();
    if result.len() != 1{
        //^feh
        todo!() // should never happen if correct things are true
    }
    let Account { displayname: _, username: _, creation_date: _, location: _, email:_, page: _, state: _, password: password_db, password_salt: salt, balance: _ } = &result[0];
    
    if !verify_password(&password_entered, password_db, salt).unwrap() {
        //^feh incorrect passwords
        todo!()
    }

    query::<()>(&mut db, "DELETE accounts WHERE username = $username;", ("username", &username)).await.unwrap();

    HttpResponse::SeeOther().append_header((actix_web::http::header::LOCATION, "/")).body(HOMEPAGE)
}











#[get("/settings/funds")]
pub async fn funds() -> impl Responder{
    // HttpResponse::Ok().body(UPLOAD)
    todo!() as HttpResponse
}

#[derive(serde::Serialize, serde::Deserialize)]
struct FundData{
    changed_funds: u64,
    password: String,
    //make an abstraction based on parts and forms and links in the js and buttons
    add: bool,
}


#[derive(serde::Serialize, serde::Deserialize)]
struct ChangeFundData{
    changed_funds: u64,
    username: String,
}


#[post("/settings/funds/change")]
async fn deposit(form: Form<FundData>, data: web::Data<AppData>, identity: Option<Identity>) -> impl Responder{
    let FundData { changed_funds, password, add } = form.into_inner();
    let username = retrieve_user(identity.unwrap()).unwrap();

    let mut db = data.db.lock().await;
    
    let surrealql = "SELECT * FROM accounts WHERE username=$username;";
    let res = query::<Account>(&mut db, surrealql, ("username", &username)).await.unwrap();
    let res = res.get(0).unwrap().as_ref().unwrap();
    if res.len() != 1{

    }
    let res = res.get(0).unwrap();

    if !verify_password(&password, &res.password, &res.password_salt).unwrap(){
        return todo!();
    }


    // let url = format!("https://www.paypal.com/sdk/js?client-id={}&currency=USD", 69696969);
    let surrealql = 
    format!("UPDATE accounts SET balance {}= $balance WHERE username=$username;",
        if add {
            "+"
        }
        else {
            "-"
        }
    );
    query_value(&mut db, &surrealql, ChangeFundData{username, changed_funds}).await.unwrap();
    HttpResponse::SeeOther().append_header((actix_web::http::header::LOCATION, "/settings")).body(SETTINGS)
}





#[get("/settings/funds/transfer")]
pub async fn transfer_funds() -> impl Responder{
    HttpResponse::Ok().body(TRANSFER)
}


#[derive(serde::Serialize, serde::Deserialize)]
struct CreditsData{
    credits: u64,
    to_username: String,
    self_password: String,
    //make an abstraction based on parts and forms and links in the js and buttons
    //add: bool,
}


#[derive(serde::Serialize, serde::Deserialize)]
pub struct TransferData{
    credits: u64,
    to_username: String,
    self_username: String,
}


#[post("/settings/funds/transfer/form")]
async fn transfer(form: Form<CreditsData>, data: web::Data<AppData>, identity: Option<Identity>) -> impl Responder{
    
    let CreditsData { credits, self_password, to_username } = form.into_inner();
    let self_username = retrieve_user(identity.unwrap()).unwrap();


    let mut db = data.db.lock().await;
    let res2 = query::<Account>(&mut db, "SELECT * FROM accounts WHERE username = $username;", ("username", &to_username)).await.unwrap();
    let result = res2.get(0).unwrap().as_ref().unwrap();
    if result.len() != 1 {
        //^feh
        //account does not exist
        todo!()
    }
    let password = &result.get(0).unwrap().password;
    let password_salt = &result.get(0).unwrap().password_salt;

    if !verify_password(&self_password, password, password_salt).unwrap(){
        //^feh
        todo!()
    }

    let transferdata = TransferData{credits, self_username, to_username};

    let surrealql = "
    UPDATE accounts SET balance -= $credits WHERE username = $self_username;
    UPDATE accounts SET balance += $credits WHERE username = $to_username;
    ";
    query_value(&mut db, surrealql, transferdata).await.unwrap();

    HttpResponse::SeeOther().append_header((actix_web::http::header::LOCATION, "/settings/funds/transfer")).body(TRANSFER)
}



use super::signup::{EmailTransmitter, email_transmission_transmit, email_transmission_receive, transmission_receive, transmission_transmit};
fn settings_transmission_transmit(session: &actix_session::Session, unhashed_code: String) -> Result<(), Box<dyn std::error::Error>>{
    email_transmission_transmit("settings", session, unhashed_code)
}

fn settings_transmission_receive(session: &actix_session::Session) -> Result<EmailTransmitter, Box<dyn std::error::Error>>{
    email_transmission_receive("settings", session)
}

#[get("/settings/email")]
pub async fn email_change(identity: Option<Identity>) -> impl Responder{
    if identity.is_none(){
        return HttpResponse::Ok().body(NOLOG);
    }
    HttpResponse::Ok().body(super::sites::EMAIL_CHANGE)
}


#[derive(serde::Serialize, serde::Deserialize)]
pub struct EmailData{
    e_old: String,
    e_new: String,
}

#[post("/settings/email/form")]
pub async fn settings_email(identity: Option<Identity>, form: Form<EmailData>, app: Data<AppData>, session: Session) -> impl Responder{
    let EmailData { e_old: current_email_input, e_new: new_email } = form.into_inner();
    // let current_email_stored =
    let mut db = app.db.lock().await;
    let q1 = query::<Account>(&mut *db, "SELECT * FROM accounts WHERE username=$username;", ("username", retrieve_user(identity.unwrap()).unwrap())).await.unwrap();
    let q2 = q1.get(0).unwrap().as_ref().unwrap().get(0).unwrap();
    if q2.email != current_email_input{
        //^feh
        return HttpResponse::Conflict().finish();
    }
    //use current_email_input to email
    use rand::Rng;
    let code = rand::thread_rng().gen_range(100000..1000000);
    settings_transmission_transmit(&session, code.to_string()).unwrap();
    settings_verification_email(&q2.email, &q2.displayname, &new_email, code).unwrap();
    transmission_transmit("set", &session, new_email).unwrap();

    HttpResponse::Ok().body(EMAIL_CHANGE_VERIFY)
}

fn settings_verification_email(email: &String, displayname: &String, new_email: &String, code: i32) -> anyhow::Result<lettre::transport::smtp::response::Response>{
    let body = format!("Dear {},\nYour account has been sent a request to change emails from {} to {}. Your verification code is {}.", displayname, email, new_email, code);
    email_user(email, "Choredom - Request to Change Emails", body)
}

#[post("/ve_set")]
pub async fn home_redirect_settings(session: Session, code: Form<super::signup::Code>, identity: Option<Identity>, data: Data<AppData>) -> impl Responder{
    let transmitter = settings_transmission_receive(&session).unwrap();
    //Remove it one case yet obtain it in another
    let new_email: String = transmission_receive("set", &session).unwrap();

    if !verify_password(&code.into_inner().code.to_string(), &transmitter.hashed_code, &transmitter.salt).unwrap(){
        //^feh
        return HttpResponse::Conflict().finish();
    }

    let mut db = data.db.lock().await;
    query_value(&mut db, "UPDATE accounts SET email = $email;", ("email", new_email)).await.unwrap();

    HttpResponse::SeeOther().append_header((actix_web::http::header::LOCATION, "/")).body(HOMEPAGE)
}





#[post("/settings/pics-pfp")]
pub async fn pics_pfp(form: Multipart, user: Option<Identity>, data: Data<AppData>) -> impl Responder{
    let user = retrieve_user(user.unwrap()).unwrap();

    let mut db = data.db.lock().await;

    process_multipart(form, format!("pfp/{user}")).await.unwrap();
    let filename = todo!() as String;
    let url = format!("/temp/pfp/{user}/{filename}");

    let _  = query_value(&mut db, "UPDATE accounts SET page.pfp_url = $url;", ("url", url)).await.unwrap();

    todo!() as HttpResponse
}


#[post("/settings/pics-bio")]
pub async fn pics_bio(form: Multipart, user: Option<Identity>) -> impl Responder{
    let user = retrieve_user(user.unwrap()).unwrap();
    // let mut db = data.db.lock().await; , data: Data<AppData>
    let paths = std::fs::read_dir("./").unwrap();

    let mut file_count = 0;
    for path in paths {
        let path = path.unwrap().path();
        if path.is_file() {
            file_count += 1;
        }
    }

    if file_count >= 3{
        return HttpResponse::BadRequest().body("No uploading over 3!");
    }

    process_multipart(form, format!("bio/{user}")).await.unwrap();

    HttpResponse::Ok().finish()
}





















#[get("/contacts")]
pub async fn dispute_management(identity: Option<Identity>) -> impl Responder{
    if identity.is_none(){
        return HttpResponse::Ok().body(NOLOG);
    }
    HttpResponse::Ok().body(CONTACT)
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ContactsInfo{
    username: String,
    email: String,
    title: String,
    message: String,
}

#[post("/contacts/form")]
pub async fn contacts_form(data: Data<AppData>, form: Form<ContactsInfo>) -> impl Responder{
     let surrealql = r#"
    BEGIN TRANSACTION;
        LET $id = (SELECT id FROM accounts WHERE username=$username)[0].id;
        CREATE disputes SET email = $email, title = $title, message = $message, user = type::thing("accounts", $id);
    COMMIT TRANSACTION;"#;
    //if there is no account it will be -> id: account:NONE
    query_value(&mut * data.db.lock().await, surrealql, form.into_inner()).await.unwrap();

    HttpResponse::SeeOther().append_header((actix_web::http::header::LOCATION, "/contacts")).body(CONTACT)
}