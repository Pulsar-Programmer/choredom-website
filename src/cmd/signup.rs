use super::sites::{SIGNUP, EMAIL, LOGIN, HOMEPAGE};
use crate::AppData;
use crate::db::{query, query_value, transmission_transmit, transmission_receive};
use actix_web::http::header;
use actix_web::{HttpMessage, HttpRequest, Responder, HttpResponse, get, web::{Form, self}, post};
use futures::executor::enter;
use lettre::transport::smtp::response::Response;
use actix_session::{Session, SessionGetError, SessionInsertError};
use rand::Rng;

#[derive(serde::Deserialize)]
pub struct SignupTransmitter{
    pub code: i64,
}

#[derive(serde::Deserialize)]
pub struct SignupData {
    pub email: String,
    pub password: String,
    pub username: String,
    pub displayname: String,
    pub location: String,
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
    pub balance: usize,
}
impl Account{
    pub fn new(username: String, displayname: String, password: String, password_salt: String, email: String, location: String) -> Self {
        Self { 
            displayname, 
            username, 
            creation_date: chrono::Utc::now(), 
            email, 
            password, 
            balance: 0, // divide by 10 to account for usize and not float
            page: AccountPage::new(),
            state: AccountState::Consumer,
            location,
            password_salt,
        }
    }
}
#[derive(serde::Serialize, Debug, serde::Deserialize, Clone)]
pub enum AccountState{
    Consumer,
    Pending,
    Worker,
}
impl AccountState{
    pub fn as_str(&self) -> &str{
        match self{
            AccountState::Consumer => "Consumer",
            AccountState::Pending => "Pending",
            AccountState::Worker => "Worker",
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
}
impl AccountPage{
    fn new() -> Self{
        Self{ 
            pfp_url: String::from("https://scontent-bos5-1.xx.fbcdn.net/v/t1.6435-9/95831445_10158064886431023_5042264117713305600_n.png?_nc_cat=111&ccb=1-7&_nc_sid=174925&_nc_ohc=jHdUksJywWcAX9BT5L0&_nc_ht=scontent-bos5-1.xx&oh=00_AfDnQ6lMQYJNm3VoLJiExu-JdGTp9T585V3NfmnukAornw&oe=64E0D75B"),
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
pub async fn verify_email(session: Session, app_data: web::Data<AppData>, form: Form<SignupData>, request: HttpRequest) -> impl Responder{
    let SignupData { email: to_email, password, username, displayname, location } = form.into_inner();
    let to_email = to_email.trim();
    let mut db = app_data.db.lock().await;
    let res2 = query::<Account>(&mut db, "SELECT * FROM accounts WHERE username = type::string($username);", Some(("username", &username))).await.unwrap();
    let result = res2.get(0).unwrap().as_ref().unwrap();
    let len1 = result.len();
    let res2 = query::<Account>(&mut db, "SELECT * FROM accounts WHERE email = type::string($email);", Some(("email", &to_email))).await.unwrap();
    let result = res2.get(0).unwrap().as_ref().unwrap();
    let len2 = result.len();
    if len1 >= 1 {
        //error , bad username OR could be an error with MORE THAN ONE username
        //^feh
        todo!()
    }
    if len1 != len2{
        //^feh
        todo!()
    }
    let code = rand::thread_rng().gen_range(100000..1000000);
    transmission_transmit("signup", &session, code).unwrap();
    confirmation_email(&to_email, &displayname, code).unwrap();

    let (password, salt) = password_hash_argon2(password).unwrap();

    let account: Account = Account::new(username.clone(), displayname , password, salt.to_string(), to_email.to_string(), location);

    // let mut db = app_data.db.lock().unwrap();

    query_value(&mut db, r#"
    CREATE accounts
    SET
    username = type::string($username),
    displayname = type::string($displayname),
    creation_date = $creation_date,
    email = type::string($email),
    page = $page,
    state = $state,
    password = $password,
    password_salt = $password_salt,
    balance = $balance,
    location = type::string($location);
    "#, Some(account)).await.unwrap();

    login_user(session, &username);
    HttpResponse::Ok().body(EMAIL)
}

#[post("/ve")]
pub async fn home_redirect(session: Session, code: Form<Code>) -> impl Responder{
    // println!("{} ; {}", code.0.code, *app_data.code.lock().unwrap());
    let true_code: i64 = transmission_receive("signup", &session).unwrap();
    if code.into_inner().code != true_code{
        //^feh
        return HttpResponse::SeeOther().append_header((header::LOCATION, "/")).body(SIGNUP)
        // todo!()
    }
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
    let smtp_key = "pjefpqhvsxmzomjf"; //app password
    let from_email: &str = "choredom3@quannt.net";
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
    //Send email?
    let LoginData { email, password } = form.into_inner();
    let email = email.trim();
    let mut db = data.db.lock().await;
    let result = query::<Account>(&mut db, "SELECT * FROM accounts WHERE email = type::string($email);", Some(("email", email))).await.unwrap();
    let result = result.get(0).unwrap().as_ref().unwrap();
    let len = result.len();
    if len > 1{
        //^feh
        todo!() // should never happen if correct things are true
    }
    else if len < 1{
        // ^feh
        return HttpResponse::Ok().body(SIGNUP)
    }
    let account = result.get(0).unwrap();
    // let password = 

    if !verify_password(&password, &account.password, &account.password_salt).unwrap(){
        // ^feh
        HttpResponse::Ok().body(LOGIN)
    }
    else{
        // confirmation_email(&account.email, &account.display_name, code); 
        login_user(session, &account.username);
        HttpResponse::SeeOther().append_header((header::LOCATION, "/")).body(HOMEPAGE)
    }
}

#[post("/signout")]
pub async fn signout(session: Session) -> impl Responder{
    println!("Goodbye: {:?}!", logout_user(session));
    HttpResponse::SeeOther().append_header((header::LOCATION, "/")).body(HOMEPAGE)
}

pub fn login_user(session: Session, username: &str) -> Result<(), SessionInsertError>{
    // session.renew();
    session.insert("username", username)
}

pub fn retrieve_user(session: Session) -> Result<Option<String>, SessionGetError>{
    session.get("username")
}

pub fn logout_user(session: Session) -> Option<String>{
    session.remove("username")
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

use password_hash::{PasswordHash, PasswordVerifier};

pub fn verify_password(entered_password: &str, stored_password: &str, salt: &str) -> anyhow::Result<bool> {
    
    let salt = SaltString::from_b64(salt)?;
    
    let argon2 = Argon2::default();


    let entered_password_hash = argon2.hash_password(entered_password.as_bytes(), &salt)?;
    
    println!("{}, {}, {}", stored_password, entered_password_hash, entered_password);

    
    Ok(argon2.verify_password(stored_password.as_bytes(), &entered_password_hash).is_ok())
}
