use actix_web::{get, post, web::{self, Form, Query}, App, HttpResponse, HttpServer, Responder};
pub const WEB: &'static str = include_str!("../html/index.html");
pub const CREATE: &'static str = include_str!("../html/create.html");
pub const COOKIE: &'static str = include_str!("../html/cookie.html");
use sqlite::State;

#[post("/mason")]
async fn login(form: web::Form<FormData>) -> impl Responder {
    // println!("{}\n{}", form.username, form.password);
    let connection = sqlite::Connection::open("data/test.db").unwrap();

    let mut statement = connection
        .prepare("SELECT * FROM users WHERE username = ?")
        .unwrap()
        .bind(1, &*form.username)
        .unwrap();

    let mut pk = 0;
    if let State::Row = statement.next().unwrap() {
        println!("username = {}", statement.read::<String>(0).unwrap());
        println!("password = {}", statement.read::<String>(1).unwrap());
        pk = statement.read::<i64>(2).unwrap();
        println!("pk = {}", pk)
    }

    HttpResponse::Found()
        .insert_header(("Location", format!("/cookie.html?pk={}", pk)))
        .body(WEB)
}

#[post("/create_account")]
async fn create_account(form: web::Form<FormData>) -> impl Responder {
    let connection = sqlite::Connection::open("data/test.db").unwrap();

    let FormData { username, password } = form.0;
    connection
        .execute(format!(
            "INSERT INTO users (username, password) VALUES ('{username}', '{password}')"
        ))
        .unwrap();
    // let mut statement = connection
    //     .prepare("INSERT INTO users (username, password) VALUES (?, ?)")
    //     .unwrap()
    //     .bind(1, &*form.username)
    //     .unwrap()
    //     .bind(2, &*form.password)
    //     .unwrap();

    // if let State::Done = statement.next().unwrap(){
    //     println!("Account successfully processed.")
    // }

    HttpResponse::Ok().body(CREATE)
}

#[post("/update_leaderboard")]
async fn update_leaderboard(f: web::Json<UserScore>) -> impl Responder{
    let connection = sqlite::Connection::open("data/test.db").unwrap();

    connection
    .execute(format!("
    UPDATE leaderboard SET rscore = {}, iscore = {}
    WHERE UserID = {}
    ", f.rscore, f.iscore, f.userid)).unwrap();
    // while let State::Row = stmt2.next().unwrap(){
        
    // }
    
    HttpResponse::Ok()
}

#[get("/getuserscore/{pk}")]
async fn getuserscore(pk: web::Path<String>) -> impl Responder {
    let connection = sqlite::Connection::open("data/test.db").unwrap();

    let mut stmt = connection
    .prepare("
    SELECT * FROM leaderboard l
    WHERE UserID = ?
    ").unwrap().bind(1, pk.parse::<i64>().unwrap()).unwrap();

    println!("PK: {pk}");

    if let State::Row = stmt.next().unwrap(){
        let user =UserScore {
            userid: stmt.read::<i64>(0).unwrap(),
            name: String::new(),
            rscore: stmt.read::<i64>(1).unwrap(),
            iscore: stmt.read::<i64>(2).unwrap(),
        };
        println!("Point 1");
        return HttpResponse::Ok().body(serde_json::to_string(&user).unwrap());
    }
    println!("Point 2");

    return HttpResponse::NotFound().finish();
}

#[get("/leaderboard")]
async fn top_10_leaderboard() -> impl Responder {
    let connection = sqlite::Connection::open("data/test.db").unwrap();

    let mut statement = connection
        .prepare(
            "SELECT UserID, pk, RScore, IScore, username
        FROM leaderboard  l
        RIGHT JOIN users u ON u.pk = l.UserID
        ORDER BY l.RScore DESC, l.IScore DESC
        LIMIT 10",
        )
        .unwrap();
    let mut v = Vec::new();

    while let State::Row = statement.next().unwrap() {
        v.push(UserScore {
            userid: statement.read::<i64>(0).unwrap(),
            rscore: statement.read::<i64>(2).unwrap(),
            iscore: statement.read::<i64>(3).unwrap(),
            name: statement.read::<String>(4).unwrap(),
        });
        // println!("username = {}", );
        // println!("password = {}", statement.read::<String>(1).unwrap());
        // println!("pk = {}", statement.read::<i64>(2).unwrap())
    }
    // HttpResponse::Ok().body(COOKIE)
    // unimplemented!()
    serde_json::to_string(&v)
}

#[get("/cookie.html")]
async fn cookie() -> impl Responder {
    HttpResponse::Ok().body(COOKIE)
}

#[get("/create.html")]
async fn create() -> impl Responder {
    HttpResponse::Ok().body(CREATE)
}

#[get("/")]
async fn main_page() -> impl Responder {
    HttpResponse::Ok().body(WEB)
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(WEB)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(actix_files::Files::new("/static", "./static").show_files_listing())
            // .service(actix_files::Files::new("/static", "./static").show_files_listing())
            .service(greet)
            .service(login)
            .service(main_page)
            .service(create)
            .service(create_account)
            .service(cookie)
            .service(top_10_leaderboard)
            .service(update_leaderboard)
            .service(getuserscore)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
#[derive(serde::Deserialize)]
struct FormData {
    username: String,
    password: String,
}
#[derive(serde::Deserialize, serde::Serialize)]
struct UserScore {
    userid: i64,
    name: String,
    rscore: i64,
    iscore: i64,
}


#[test]
fn test_getuserscore() {

    let connection = sqlite::Connection::open("data/test.db").unwrap();

    let mut stmt = connection
    .prepare("
    SELECT * FROM leaderboard l
    WHERE UserID = ?
    ").unwrap().bind(1, 1).unwrap();

    if let State::Row = stmt.next().unwrap(){
        let user = (
            stmt.read::<i64>(0).unwrap(),
            stmt.read::<i64>(1).unwrap(),
            stmt.read::<i64>(2).unwrap(),
        );
        println!("{}", user.1);
    }
    
}