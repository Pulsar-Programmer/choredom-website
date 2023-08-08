use super::sites::{SIGNUP, EMAIL, LOGIN, HOMEPAGE};
use crate::{AppData, Transmitter};
use crate::structs::Money;
use crate::db::{dissolve, query, query_value};
use actix_web::web::Json;
use actix_web::{HttpMessage, HttpRequest, Responder, HttpResponse, get, web::{Form, self}, post};
use actix_identity::Identity;
use rand::Rng;


pub struct SignupTransmitter{
    pub state: AccountState,
    pub code: i64,
}
impl Transmitter for SignupTransmitter{}
impl Default for SignupTransmitter{
    fn default() -> Self {
        Self { state: AccountState::Consumer, code: 0 }
    }
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

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Account{
    pub display_name: String,
    pub username: String, //USERNAME STORED IN DB AS ID
    pub creation_date: chrono::DateTime<chrono::Utc>,
    // pub last_location: Location,
    pub location: String, //just a string for now

    pub email: String,
    pub page: AccountPage,
    pub state: AccountState,

    pub password: String,
    pub balance: Money,
}
impl Account{
    pub fn new(username: String, display_name: String, password: String, email: String, location: String) -> Self {
        Self { 
            display_name, 
            username, 
            creation_date: chrono::Utc::now(), 
            email, 
            password, 
            balance: Money(0.), 
            page: AccountPage::new(),
            state: AccountState::Consumer,
            location,
        }
    }
}
#[derive(serde::Serialize, Debug, serde::Deserialize)]
pub enum AccountState{
    Consumer,
    Pending,
    Worker,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AccountPage{
    pfp_url: String,
    avg_rating: f64,
    reviews: Vec<super::profile::RatingData>,
    bio: String,
}
impl AccountPage{
    fn new() -> Self{
        Self{ 
            pfp_url: String::from("https://scontent-bos5-1.xx.fbcdn.net/v/t1.6435-9/95831445_10158064886431023_5042264117713305600_n.png?_nc_cat=111&ccb=1-7&_nc_sid=174925&_nc_ohc=jHdUksJywWcAX9BT5L0&_nc_ht=scontent-bos5-1.xx&oh=00_AfDnQ6lMQYJNm3VoLJiExu-JdGTp9T585V3NfmnukAornw&oe=64E0D75B"),
            avg_rating: 0., reviews: Vec::new(),
            bio: String::new(),
        }
    }
}

#[get("/signup")]
pub async fn signup() -> impl Responder{
    HttpResponse::Ok().body(SIGNUP)
}

#[post("/verify-email")]
pub async fn verify_email(app_data: web::Data<AppData>, form: Json<SignupData>, request: HttpRequest) -> impl Responder{
    let SignupData { email: to_email, password, password2, username, displayname, location } = form.0;

    if password != password2{
        // ^feh 1
        return HttpResponse::Ok().body(SIGNUP);
    }

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
    
    let mut code = app_data.transmitters.signup.lock().await;
    let codea = rand::thread_rng().gen_range(100000..1000000);
    (*code).code = codea;
    let body = format!("Welcome to Choredom, {}. Your verification code is {}", displayname, codea);

    email(&to_email, "Welcome to Choredom!", body);

    let account: Account = Account::new(username.clone(), displayname , password, to_email, location);

    // let mut db = app_data.db.lock().unwrap();

    dissolve(query_value(&mut db, r#"
    CREATE accounts
    SET
    username = type::string($username),
    display_name = type::string($display_name),
    creation_date = $creation_date,
    email = type::string($email),
    page = $page,
    state = $state,
    password = type::string($password),
    balance = $balance,
    location = type::string($location);
    "#, Some(account)).await, 0);

    login_user(request, username);

    HttpResponse::Ok().body(EMAIL)
}

#[post("/")]
pub async fn settings_redirect(app_data: web::Data<AppData>, code: Form<Code>) -> impl Responder{
    // println!("{} ; {}", code.0.code, *app_data.code.lock().unwrap());
    if code.0.code != app_data.transmitters.signup.lock().await.code{
        //^feh
        // HttpResponse::Ok().body(EMAIL)
        todo!()
    }
    else{
        HttpResponse::Ok().body(HOMEPAGE)
    }
}





fn email(to_email: &str, subject: &str, body: String){
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::{SmtpTransport, Transport};
    use lettre::Message;

    // let smtp_key: &str = "Brokies129gg";
    let smtp_key = "pjefpqhvsxmzomjf"; //app password
    let from_email: &str = "business@quannt.net";
    let host: &str = "smtp.gmail.com";

    let email: Message = Message::builder()
        .from(from_email.parse().unwrap())
        .to(to_email.parse().unwrap())
        .subject(subject)
        .body(body)
        .unwrap();

    let creds: Credentials = Credentials::new(from_email.to_string(), smtp_key.to_string());

    // Open a remote connection to gmail
    let mailer: SmtpTransport = SmtpTransport::relay(&host)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => {println!("Email sent successfully!");},
        Err(e) => println!("{e}"), //invalid email ^feh 4
    };
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
pub async fn signin(form: Form<LoginData>, data : web::Data<AppData>, request: HttpRequest) -> impl Responder{
    //Send email?
    let LoginData { email, password } = form.0;
    let mut db = data.db.lock().await;
    let result = query::<Account>(&mut db, "SELECT * FROM accounts WHERE email = type::string($email);", Some(("email", email))).await.unwrap();
    let result = result.get(0).unwrap().as_ref().unwrap();
    let len = result.len();
    if len > 1{
        //^feh
        todo!() // should never happen if correct things are true
    }
    else if len < 1{
        // ^feh 3
        return HttpResponse::Ok().body(SIGNUP)
    }
    let account = result.get(0).unwrap();
    if account.password != password{
        // ^feh 2
        HttpResponse::Ok().body(LOGIN)
    }
    else{
        login_user(request, account.username.clone());
        HttpResponse::Ok().body(HOMEPAGE)
    }
}


#[get("/")]
async fn index(user: Option<Identity>) -> impl Responder {
    if let Some(user) = user {
        format!("Welcome! {}", user.id().unwrap())
    } else {
        "Welcome Anonymous!".to_owned()
    }
}

fn login_user(request: HttpRequest, username: String) -> impl Responder {
    // Some kind of authentication should happen here
    // e.g. password-based, biometric, etc.
    // [...]
    // attach a verified user identity to the active session
    Identity::login(&request.extensions(), username).unwrap();
    HttpResponse::Ok()
}

#[post("/logout")]
async fn logout(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::Ok().body("Logged out.")
}