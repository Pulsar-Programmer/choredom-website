use super::sites::*;
use crate::{AppData, Transmitter};
use crate::structs::{Money};
use crate::db::{dissolve, query, query_value};
use actix_web::{Responder, HttpResponse, get, web::{Form, self}, post};
use rand::Rng;

pub struct SignupTransmitter{
    pub state: AccountState,
    pub code: i64,
}
impl Transmitter for SignupTransmitter{}

#[derive(serde::Deserialize)]
pub struct SignupData {
    pub email: String,
    pub password: String,
    pub password2: String, //send through frontend differently
    pub username: String,
    pub displayname: String,
    // pub location: String,
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

    pub email: String,
    pub data: AccountData,
    pub page: AccountPage,
    pub state: AccountState,

    pub password: String,
    pub balance: Money,
}
impl Account{
    pub fn new(username: String, display_name: String, password: String, email: String) -> Self {
        Self { 
            display_name, 
            username, 
            creation_date: chrono::Utc::now(), 
            email, 
            data: AccountData::new(), 
            password, 
            balance: Money(0.), 
            page: AccountPage::new(),
            // last_location: todo!(),
            state: AccountState::Consumer,
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
pub struct AccountData{
    rating: f64,
    reviews: Vec<String>,
}
impl AccountData{
    fn new() -> Self{
        Self{ rating: 0., reviews: Vec::new() }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AccountPage{
    pfp_url: String,
}
impl AccountPage{
    fn new() -> Self{
        Self{ 
            pfp_url: String::from("https://scontent-bos5-1.xx.fbcdn.net/v/t1.6435-9/95831445_10158064886431023_5042264117713305600_n.png?_nc_cat=111&ccb=1-7&_nc_sid=174925&_nc_ohc=jHdUksJywWcAX9BT5L0&_nc_ht=scontent-bos5-1.xx&oh=00_AfDnQ6lMQYJNm3VoLJiExu-JdGTp9T585V3NfmnukAornw&oe=64E0D75B"),  
        }
    }
}

#[get("/signup")]
pub async fn signup() -> impl Responder{
    HttpResponse::Ok().body(SIGNUP)
}

#[post("/verify-email")]
pub async fn verify_email(app_data: web::Data<AppData>, form: Form<SignupData>) -> impl Responder{
    let SignupData { email: to_email, password, password2, username, displayname } = form.0;

    if password != password2{
        // ^feh 1
        return HttpResponse::Ok().body(SIGNUP);
    }
    
    let mut code = app_data.transmitters.0.lock().unwrap();
    let codea = rand::thread_rng().gen_range(100000..1000000);
    (*code).code = codea;
    let body = format!("Welcome to Choredom, {}. Your verification code is {}", displayname, codea);

    email(&to_email, "Welcome to Choredom!", body);

    let cookie = super::login::login_cookie(&username);

    let account: Account = Account::new(username, displayname , password, to_email);

    let mut db = app_data.db.lock().unwrap();

    dissolve(query_value(&mut db, r#"
    CREATE accounts
    SET
    username = type::string($username)
    display_name = type::string($display_name),
    creation_date = $creation_date,
    email = type::string($email),
    data = $data,
    page = $page,
    state = $state,
    password = type::string($password),
    balance = $balance;
    "#, Some(account)).await, 0);

    let mut resp = HttpResponse::Ok().body(EMAIL);
    if let Err(e) = resp.add_cookie(&cookie){
        return HttpResponse::Ok().body(e.to_string())
    }
    resp
    
}

#[post("/upload")]
pub async fn upload(app_data: web::Data<AppData>, code: Form<Code>) -> impl Responder{
    // println!("{} ; {}", code.0.code, *app_data.code.lock().unwrap());
    if code.0.code != app_data.transmitters.0.lock().unwrap().code{
        HttpResponse::Ok().body(EMAIL)
    }
    else{
        HttpResponse::Ok().body(UPLOAD)
    }
}

#[post("/upload-auth")]
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
    Ok(HttpResponse::Ok().into())
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