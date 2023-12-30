use super::sites::{SIGNUP, EMAIL, LOGIN, HOMEPAGE, EMAIL_LOG};
use crate::AppData;
use crate::db::{query_once, sole_query};
use actix_identity::Identity;
use actix_web::http::header;
use actix_web::{HttpMessage, HttpRequest, Responder, HttpResponse, get, web::{Form, self}, post};
use chrono::{Utc, Duration};
use lettre::transport::smtp::response::Response;
use actix_session::Session;
use rand::Rng;
use crate::RainError as r;

#[derive(serde::Deserialize)]
pub struct SignupTransmitter{
    pub code: i64,
}

#[derive(serde::Deserialize)]
pub struct SignupData {
    email: String,
    password: String,
    username: String,
    displayname: String,
    location: String,
}

#[derive(serde::Deserialize)]
pub struct Code{
    pub code: i64
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Account{
    pub displayname: String,
    pub username: String, //USERNAME STORED IN DB AS ID
    pub creation_date: chrono::DateTime<chrono::Utc>,
    // pub last_location: Location,
    pub location: String, //just a string for now

    pub email: String,
    pub page: AccountPage,
    pub state: AccountState,

    pub password: String,
    pub password_salt: String,
    pub balance: u64,
}
impl Account{
    pub fn new(username: String, displayname: String, password: String, password_salt: String, email: String, location: String) -> Self {
        Self { 
            displayname, 
            username, 
            creation_date: chrono::Utc::now(), 
            email, 
            password, 
            balance: 0, // divide by 10 to account for u64 and not float
            page: AccountPage::new(),
            state: AccountState::NonVerified,
            location,
            password_salt,
        }
    }
}
#[derive(serde::Serialize, Debug, serde::Deserialize, Clone)]
pub enum AccountState{
    NonVerified,
    PendingVerification,
    Verified,
}
impl AccountState{
    pub fn as_str(&self) -> &str{
        match self{
            AccountState::NonVerified => "NonVerified",
            AccountState::PendingVerification => "PendingVerification",
            AccountState::Verified => "Verified",
        }
    }
}
impl ToString for AccountState{
    fn to_string(&self) -> String {
        String::from(self.as_str())
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct AccountPage{
    pub pfp_url: String,
    pub avg_rating: rust_decimal::Decimal,
    pub reviews: Vec<super::profile::PageRatingData>,
    pub bio: String,
    // pub bio_images: [String; 3],
}
impl AccountPage{
    fn new() -> Self{
        let options = ["yellow", "blue", "red", "green", "pink"];
        let u = rand::thread_rng().gen_range(0..5);
        let pfp_url = format!("/src-web/assets/stdpfps/{}.png", options[u]);
        Self{
            pfp_url,
            avg_rating: rust_decimal::Decimal::ZERO, reviews: Vec::new(),
            bio: String::new(),
        }
    }
}

#[get("/signup")]
pub async fn signup() -> impl Responder{
    HttpResponse::Ok().body(SIGNUP)
}

#[post("/verify-email")]
pub async fn verify_email(session: Session, app_data: web::Data<AppData>, form: Form<SignupData>) -> impl Responder{
    let SignupData { email: to_email, password, username, displayname, location } = form.into_inner();
    let true = satisfies_displayname(&displayname) else { return r::for_html("Invalid displayname!")};
    let true = satisfies_username(&username) else { return r::for_html("Invalid username!")};
    let true = satisfies_email(&to_email) else { return r::for_html("Invalid email!")};
    let true = satisifies_password(&password) else { return r::for_html("Invalid password!")};
    //how much let is too much let? when does pattern matching become TOO op?

    let to_email = to_email.trim();
    let mut db = app_data.db.lock().await;
    let Ok(result) = query_once::<Account>(&mut db, "SELECT * FROM accounts WHERE username = $username;", ("username", &username)).await else { return r::for_html_stderr()};
    let len1 = result.len();
    let Ok(result) = query_once::<Account>(&mut db, "SELECT * FROM accounts WHERE email = $email;", ("email", &to_email)).await else { return r::for_html_stderr()};
    let len2 = result.len();
    if len1 >= 1 {
        return r::for_html("That username is taken. Choose a different username.")
    }
    if len1 != len2{
        return r::for_html("That email is taken. Choose a different email.")
    }
    let code = rand::thread_rng().gen_range(100000..1000000);
    // transmission_transmit("signup", &session, code).unwrap();
    let Ok(..) = signup_transmission_transmit(&session, code.to_string()) else { return r::for_html_stderr() };
    let Ok(..) = confirmation_email(to_email, &displayname, code) else { return r::for_html_stderr() };

    let Ok((password, salt)) = password_hash_argon2(password) else { return r::for_html_stderr() };

    let account: Account = Account::new(username.clone(), displayname , password, salt.to_string(), to_email.to_string(), location);

    let Ok(..) = transmission_transmit("account", &session, account) else { return r::for_html_stderr() };

    HttpResponse::Ok().body(EMAIL)
}

#[post("/ve")]
pub async fn home_redirect_signup(session: Session, code: Form<Code>, data: web::Data<AppData>, request: HttpRequest) -> impl Responder{
    let Ok(transmitter) = signup_transmission_receive(&session) else { return r::for_html_stderr() };
    //Remove in one case and obtain in another
    let Ok(account) = transmission_receive::<Account>("account", &session) else { return r::for_html_stderr() };

    let Ok(passwords_match) = verify_password(&code.into_inner().code.to_string(), &transmitter.hashed_code, &transmitter.salt) else { return r::for_html_stderr() };

    if !passwords_match{
        return r::for_html("Codes don't match!")
        // return HttpResponse::SeeOther().append_header((header::LOCATION, "/signup")).body(SIGNUP)
    }
    let mut db = data.db.lock().await;

    //We want to create the account only AFTER we verify codes.

    let Ok(..) = sole_query(&mut db, r#"
    CREATE accounts
    SET
    username = $username,
    displayname = $displayname,
    creation_date = $creation_date,
    email = $email,
    page = $page,
    state = $state,
    password = $password,
    password_salt = $password_salt,
    balance = $balance,
    location = $location;
    "#, Some(&account)).await else { return r::for_html_stderr() };

    let Ok(..) = login_user(&request, account.username) else { return r::for_html_stderr() };

    // HttpResponse::TemporaryRedirect().append_header(("Location", "/")).body(HOMEPAGE)
    HttpResponse::SeeOther().append_header((header::LOCATION, "/")).body(HOMEPAGE)
}






fn confirmation_email(to_email: &str, displayname: &str, code: i64) -> anyhow::Result<Response>{
    let body = format!("Welcome to Choredom, {}. Your verification code is {}.", displayname, code);
    email_user(to_email, "Welcome to Choredom!", body)
}


pub fn email_user(to_email: &str, subject: &str, body: String) -> anyhow::Result<Response>{
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::{SmtpTransport, Transport};
    use lettre::Message;

    // let smtp_key: &str = "Brokies129gg";
    let smtp_key = "lwpcpxpvncogqaxz"; //app password
    let from_email: &str = "aaron.sachan.bang@gmail.com";
    let host: &str = "smtp.gmail.com";

    let email: Message = Message::builder()
        .from(from_email.parse()?)
        .to(to_email.parse()?)
        .subject(subject)
        .body(body)?;

    let creds: Credentials = Credentials::new(from_email.to_string(), smtp_key.to_string());

    // Open a remote connection to gmail
    let mailer: SmtpTransport = SmtpTransport::relay(host)?
        .credentials(creds)
        .build();


    //check for invalid email ^feh

    // Send the email
    mailer.send(&email).map_err(|err|err.into())


}

// #[get("/accounts")]
// async fn accounts(app_data: web::Data<AppData>) -> impl Responder{
//     let mut db = app_data.db.lock().await;
//     let res2 = query::<Account>(&mut db, "SELECT * FROM accounts;", None::<()>).await.unwrap();
//     let res1 = res2.get(0).unwrap();
//     let result = res1.as_ref().unwrap();
//     HttpResponse::Ok().body(format!("{result:?}"))
// }

#[derive(serde::Deserialize)]
pub struct LoginData{
    email: String,
    password: String,
}

#[get("/login")]
pub async fn login() -> impl Responder{
    HttpResponse::Ok().body(LOGIN)
}

#[post("/signin")] // will this work if we choose homepage instead? ERROR ERROR PLEASE SEE ME
pub async fn signin(form: Form<LoginData>, data : web::Data<AppData>, session: Session) -> impl Responder{
    let LoginData { email, password } = form.into_inner();

    // let true = satisfies_email(&email) else { return r::for_html("Invalid email!")};
    // let true = satisifies_password(&password) else { return r::for_html("Invalid password!")};
    //we don't actually need this since we match agnst the databse

    let email = email.trim();
    let mut db = data.db.lock().await;
    let Ok(result) = query_once::<Account>(&mut db, "SELECT * FROM accounts WHERE email = $email;", ("email", email)).await else { return r::for_html_stderr()};
    let Some(account) = result.get(0) else { return r::for_html("Ensure to create the account, first!")};

    let Ok(passwords_match) = verify_password(&password, &account.password, &account.password_salt) else { return r::for_html_stderr()};

    if !passwords_match{
        return r::for_html("Passwords don't match!");
    }
    
    let code = rand::thread_rng().gen_range(100000..1000000);
    let Ok(..) = login_transmission_transmit(&session, code.to_string()) else { return r::for_html_stderr()};
    let Ok(..) = confirmation_email(&account.email, &account.displayname, code) else { return r::for_html("Error sending email.") };
    let Ok(..) = transmission_transmit("log", &session, account.username.clone()) else { return r::for_html_stderr()};
    HttpResponse::Ok().body(EMAIL_LOG)
    // HttpResponse::SeeOther().append_header((header::LOCATION, "/")).body(HOMEPAGE)
}


#[post("/ve_log")]
pub async fn home_redirect_login(session: Session, code: Form<Code>, request: HttpRequest) -> impl Responder{
    let Ok(transmitter) = login_transmission_receive(&session) else{ return r::for_html_stderr()};
    //Remove in one case and obtain in another
    let Ok(username) = transmission_receive::<String>("log", &session) else { return r::for_html_stderr()};

    let Ok(passwords_match) = verify_password(&code.into_inner().code.to_string(), &transmitter.hashed_code, &transmitter.salt) else { return r::for_html_stderr()};

    if !passwords_match{
        return r::for_html("Codes don't match!")
    }
    let Ok(..) = login_user(&request, username) else { return r::for_html_stderr()};
    // HttpResponse::TemporaryRedirect().append_header(("Location", "/")).body(HOMEPAGE)
    HttpResponse::SeeOther().append_header((header::LOCATION, "/")).body(HOMEPAGE)
}


#[post("/signout")]
pub async fn signout(identity: Option<Identity>) -> impl Responder{
    let Some(identity) = identity else { return r::for_js_user("Sign in to first sign out!")};

    println!("Goodbye: {:?}!", logout_user(identity));

    HttpResponse::SeeOther().append_header((header::LOCATION, "/")).body(HOMEPAGE)
}

pub fn login_user(http_request: &HttpRequest, username: String) -> Result<Identity, actix_identity::error::LoginError>{
    // session.renew();
    Identity::login(&http_request.extensions(), username)
}

pub fn retrieve_user(identity: Identity) -> Result<String, actix_identity::error::GetIdentityError>{
    identity.id()
}

pub fn unwrap_identity(identity: Option<Identity>) -> Result<String, anyhow::Error>{
    Ok(retrieve_user(identity.ok_or(anyhow::anyhow!("The identity could not be extracted."))?)?)
}

pub fn logout_user(identity: Identity){
    identity.logout()
}

use password_hash::{SaltString, PasswordHasher};
use argon2::Argon2;
pub fn password_hash_argon2(password: String) -> anyhow::Result<(String, SaltString)>{
    
    
    let salt = SaltString::generate(&mut rand::thread_rng());

    // Create an Argon2 password hasher
    let argon2 = Argon2::default();

    // Hash the password
    let string = argon2.hash_password(password.as_bytes(), &salt)?.to_string();
    Ok((string, salt))
}

pub fn verify_password(entered_password: &str, stored_password: &str, salt: &str) -> anyhow::Result<bool> {
    
    let salt = SaltString::from_b64(salt)?;
    
    let argon2 = Argon2::default();


    let entered_password_hash = argon2.hash_password(entered_password.as_bytes(), &salt)?;
    
    Ok(stored_password == entered_password_hash.to_string())
}





#[derive(serde::Serialize, serde::Deserialize)]
///Incorporate this when transferring data.
pub struct EmailTransmitter{
    pub hashed_code: String,
    pub salt: String,
    pub time: chrono::DateTime<Utc>,
}
impl EmailTransmitter{
    pub fn new(unhashed_code: String) -> anyhow::Result<Self>{
        let (hashed_code, salt) = password_hash_argon2(unhashed_code)?;
        Ok(Self{
            hashed_code,
            salt: salt.to_string(),
            time: Utc::now(),
        })
    }
}



fn login_transmission_transmit(session: &actix_session::Session, unhashed_code: String) -> Result<(), Box<dyn std::error::Error>>{
    email_transmission_transmit("login", session, unhashed_code)
}

fn login_transmission_receive(session: &actix_session::Session) -> Result<EmailTransmitter, Box<dyn std::error::Error>>{
    email_transmission_receive("login", session)
}

fn signup_transmission_transmit(session: &actix_session::Session, unhashed_code: String) -> Result<(), Box<dyn std::error::Error>>{
    email_transmission_transmit("signup", session, unhashed_code)
}

fn signup_transmission_receive(session: &actix_session::Session) -> Result<EmailTransmitter, Box<dyn std::error::Error>>{
    email_transmission_receive("signup", session)
}

pub fn email_transmission_transmit(field:&str, session: &actix_session::Session, unhashed_code: String) -> Result<(), Box<dyn std::error::Error>>{
    let transmitter = EmailTransmitter::new(unhashed_code)?;
    transmission_transmit(field, session, transmitter)
}

pub fn email_transmission_receive(field: &str, session: &actix_session::Session) -> Result<EmailTransmitter, Box<dyn std::error::Error>>{
    let transmitter: EmailTransmitter = transmission_receive(field, session)?;
    if Utc::now() - transmitter.time >= Duration::minutes(5){
        return Err("Message not received in time!".into());
    }
    Ok(transmitter)
}





pub fn transmission_transmit<Args: serde::Serialize>(field: &str, session: &actix_session::Session, args: Args) -> Result<(), Box<dyn std::error::Error>>{
    let derived_field = format!("{}_transmitter", field);
    session.insert(derived_field, args)?;
    Ok(())
}
pub fn transmission_receive<Transmitter: serde::de::DeserializeOwned>(field: &str, session: &actix_session::Session) -> Result<Transmitter, Box<dyn std::error::Error>>{
    let derived_field = format!("{}_transmitter", field);
    let value = session.remove(&derived_field).ok_or("Failed to transmit using transmitter.")?;
    // session.purge(); I don't know how to fix this but for now it just breaks it.
    Ok(serde_json::from_str(&value)?)
}

use fancy_regex::Regex;
pub fn satisfies_username(username: &str) -> bool{
    satisfies(username, "^[A-Za-z0-9]+$")
}

pub fn satisifies_password(password: &str) -> bool{
    satisfies(password, r"^(?=.*[A-Z])(?=.*[a-z])(?=.*\d)(?=.*[!@#$%&])(?!.*\s).{8,}$")
}

pub fn satisfies_email(email: &str) -> bool{
    satisfies(email, "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+.[a-zA-Z]{2,}$")
}

pub fn satisfies_displayname(displayname: &str) -> bool{
    satisfies(displayname, "^(?! *$)[A-Za-z0-9 ]{3,20}$")
}

fn satisfies(string: &str, regex: &str) -> bool{
    // let regex = format!("/{regex}/g");
    #[allow(clippy::unwrap_used)]
    let re = Regex::new(regex).unwrap();
    re.is_match(string).unwrap_or(false)
}

// fn sanitize()