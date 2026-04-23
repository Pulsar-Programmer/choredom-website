use std::{collections::HashMap, str::FromStr};

use actix_files::NamedFile;
use actix_identity::Identity;
use actix_multipart::form::MultipartForm;
use actix_web::{get, post, Responder, HttpResponse, web::{Data, Json, Path}, HttpRequest};
use chrono::{DateTime, Utc};
use surrealdb_types::SurrealValue;
use crate::{AppData, RainError, cmd::{signup::AccountState, sites::NOLOG}, db::{query_once, query_once_option, sole_query}, img::{upload_file, verify_img}};
use super::sites::{CHAT, CHATNAV, NOUSER};
use super::signup::unwrap_identity;
use RainError as r;

///This represents a chat room with a bunch of chats.
#[derive(Debug, serde::Serialize, serde::Deserialize, SurrealValue)]
pub struct Room{
    room_id: RoomID,
    messages: Vec<ChatData>,
}

/**
This is where the user goes to enter the chats.
The frontend part will consist of a field that is a forum that posts to `/chat/send` with NO redirection.
Upon refresh, all the chats will stay because the chat messages will be added.
*/
#[get("/chats/{receiver}")]
pub async fn chats_get(receiver: Path<String>, app_data: Data<AppData>, identity: Option<Identity>) -> impl Responder{
    let Ok(sender) = unwrap_identity(identity) else {return RainError::for_html(NOLOG)};
    let Ok(Some(a)) = query_once_option::<String>(&app_data.db, "SELECT * FROM (SELECT state FROM accounts WHERE username=$username).state;", ("username", &sender)).await else { return RainError::for_html(NOUSER)};
    match AccountState::from_str(&a) {
        super::signup::AccountState::Verified => {},
        _ => {return RainError::for_html(super::sites::NOVER)}
    }

    let receiver = receiver.into_inner();
    let Ok(result) = query_once::<super::signup::Account>(&app_data.db, "SELECT * FROM accounts WHERE username=$username;", ("username", &receiver)).await else {return RainError::for_html_stderr()};
    if result.len() != 1 {
        return RainError::for_html(NOUSER);
    }
    if sender == receiver{
        return RainError::for_html("
        <!DOCTYPE html>
        <html>
        <head>
            <title>Don't send chats to yourself.</title>
        </head>
        <body>
            <h1>Access Denied</h1>
            <p>Nice try, you can't send chats to yourself/p>
        </body>
        </html>
        ")
    }
    HttpResponse::Ok().body(CHAT)
}

#[derive(serde::Serialize)]
struct ChatFrontDataPFP{
    data: ChatFrontData,
    pfpurl: String,
}

#[post("/chats_obtain")]
pub async fn chats_obtain(receiver: Json<String>, identity: Option<Identity>, data: Data<crate::AppData>) -> impl Responder{
    let Ok(sender) = unwrap_identity(identity) else { return RainError::for_js("Identity cannot be extracted.")};
    let receiver = receiver.into_inner();
    let room_id = RoomID::create([sender.clone(), receiver.clone()]);

    //build a room and send to db if one doesn't exist >> use indicies for this

    let opposite = room_id[true] == receiver;
    let useful_data = ChatDBQuery{ sender: opposite, room_id: room_id.clone() };

    // we must first redeem them as all read, since you are entering
    let Ok(_) = sole_query(&data.db, "UPDATE chats SET messages[WHERE was_read = false AND sender = $sender].was_read = true WHERE room_id = $room_id;", &useful_data).await else {return r::for_js("Error updating.")};
    let Ok(result) = query_once::<Room>(&data.db, "SELECT * FROM chats WHERE room_id = $room_id;", ("room_id", &room_id)).await else { return r::for_js("Error getting chats.")};
    let Some(result) = result.first() else {
        let room = Room{room_id, messages: Vec::new()};
        let Ok(_) = sole_query(&data.db, "CREATE chats SET room_id=$room_id, messages=$messages;", room).await else { return r::for_js("Error creating new chat room.")};
        return HttpResponse::Ok().json(Vec::<ChatData>::new());
    };
    let Room { room_id: _, messages: vec } = result;

    let mut pfp_cache: HashMap<String, String> = HashMap::new();

    let mut pfps = Vec::new();
    for ChatData { sender, ..} in vec{
        let sender = room_id[sender.to_owned()].to_owned();
        let pfpurl = if let Some(v) = pfp_cache.get(&sender) {
            v.clone()
        } else {
            let v: String = match query_once_option(
                &data.db,
                "SELECT * FROM (SELECT page.pfp_url FROM accounts WHERE username=$username).page.pfp_url;",
                ("username", &sender)
            ).await {
                Ok(Some(v)) => v,
                _ => return RainError::for_js("Error retrieving pfp_url."),
            };
            pfp_cache.insert(sender.clone(), v.clone());
            v
        };
        pfps.push(pfpurl);
    }

    let vec : Vec<ChatFrontDataPFP> = vec.iter().map(move|ChatData { timestamp, msg, sender, was_read:_ }|{
        let sender = room_id[sender.to_owned()].to_owned();
        let data = ChatFrontData { timestamp: DateTime::<Utc>::from_str(timestamp).expect("string is not time").format("%m/%d/%Y @ %H:%M").to_string(), msg: msg.to_owned(), sender };
        ChatFrontDataPFP { data, pfpurl: pfps.remove(0)}
    }).collect();

    //update the DOM the most recent chat messages (later having JavaScript add any new ones to the DOM)
    HttpResponse::Ok().json(&vec)
}












///The chat data stored in the database.
#[derive(serde::Serialize, Debug, serde::Deserialize, Clone, SurrealValue)]
pub struct ChatData{
    pub timestamp: String,
    pub msg: String,
    ///Be careful here, as we must ensure sender is in the room, as in, it is contained within the `RoomID`.
    ///There was a thought of using a boolean here to save storage, but we decided not to integrate it.
    ///Nevermind we are changing this again to a boolean to save immense storage. I am a monkie. Sorry for that.
    pub sender: bool,
    ///Here, the was_read condition is pertaining to the person opposite of the sender.
    pub was_read: bool,
}

///The chat data given to the frontend.
#[derive(serde::Serialize, Debug)]
struct ChatFrontData{
    timestamp: String,
    msg: String,
    sender: String,
}
///Just adds stuff to the DB.
#[derive(serde::Deserialize, serde::Serialize)]
struct FakeRoom{
    chat: ChatData,
    room_id: RoomID,
}
#[derive(serde::Deserialize)]
pub struct FrontSentData{
    room_title: String,
    msg: String,
}



#[derive(serde::Serialize)]
struct ChatFrontDataBundle{
    data: Vec<ChatFrontData>,
    pfpurl: String,
}
//text_message
/// This, given the msg and the sender, sends a message and logs it in the Database.
/// It then can be retrieved from the receive message function.
/// We send back the data sent so it can be displayed immediately.
#[post("/chat/send")]
pub async fn send(json: Json<FrontSentData>, identity: Option<Identity>, app: Data<crate::AppData>) -> impl Responder{
    // println!("Chat sent!");
    //Here, `json` represents the reciever and the msg intended to be sent.
    let Ok(named_sender) = unwrap_identity(identity) else { return r::for_js("Sender could not be identified.") };
    let timestamp = Utc::now();
    let FrontSentData{room_title, msg} = json.into_inner();
    let room_id = RoomID::create([room_title, named_sender.clone()]);
    let sender = named_sender == room_id[true]; //if the sender equals the room ids second index, it returns true as chosen before; otherwise it returns false correctly.

    let to_database = ChatData{timestamp: timestamp.to_string(), msg: msg.clone(), sender, was_read: false};
    let fake_room = FakeRoom{chat: to_database, room_id};

    let Ok(..) = sole_query(&app.db, "UPDATE chats SET messages += $chat WHERE room_id = $room_id;", fake_room).await else { return r::for_js("Error adding chat to room.")};

    let Ok(Some(pfpurl)) = query_once_option(&app.db, "SELECT * FROM (SELECT page.pfp_url FROM accounts WHERE username=$username).page.pfp_url;", ("username", &named_sender)).await else { return RainError::for_js("Error retrieving pfp_url.")};

    let to_frontend = ChatFrontData{ timestamp: timestamp.format("%m/%d/%Y @ %H:%M").to_string(), msg, sender: named_sender };

    let plus_pfp = ChatFrontDataPFP{ data: to_frontend, pfpurl };
    // println!("Chat bounceback: {to_frontend:?}");
    HttpResponse::Ok().json(plus_pfp)
}

#[derive(serde::Serialize, serde::Deserialize, Debug, SurrealValue)]
pub struct ChatDBGiven{
    pub messages: Vec<ChatData>,
}
#[derive(serde::Serialize, Debug)]
pub struct ChatDBQuery{
    sender: bool,
    room_id: RoomID,
}

//GIVE THE FRONTEND : Vec<ChatFrontData>
/// This uses long polling to eventually give the frontend a Vec<ChatFrontData> which is useful for adding it to the DOM.
/// The difference here is that, now, we will use the LIVE feature on SurrealDB to find the next update and use it.
/// This is opposed to the method used above when refreshing the page which simply obtains all of the Vec<ChatFrontData> rather than the new ones.
#[post("/chat/receive")]
pub async fn receive(identity: Option<Identity>, opposite: Json<String>, data: Data<AppData>) -> impl Responder{
    let Ok(same) = unwrap_identity(identity) else { return r::for_js_user("Log in to receive chats!")};
    let opposite = opposite.into_inner();
    let room_id = RoomID::create([same, opposite.clone()]);
    let opposite_unnamed = room_id[true] == opposite;
    let useful_data = ChatDBQuery{ sender: opposite_unnamed, room_id: room_id.clone() };

    //must incorporate the WHILE LET and the EVENT kind of idea to wait for the long polling to end and such and such
    // println!("{useful_data:?}");
    let Ok(res) = query_once::<ChatDBGiven>(&data.db, "SELECT messages[WHERE was_read = false AND sender = $sender] FROM chats WHERE room_id = $room_id;", &useful_data).await else{ return r::for_js("Could not select chats.")};
    let Some(dbgiven) = res.first() else {return HttpResponse::Ok().json(Vec::<ChatDBGiven>::new())};
    let chats_vec = &dbgiven.messages;
    //mark as read right before
    let Ok(..) = sole_query(&data.db, "UPDATE chats SET messages[WHERE was_read = false AND sender = $sender].was_read = true WHERE room_id = $room_id;", &useful_data).await else { return r::for_js("Could not mark chats as read.")};

    let chats_vec : Vec<ChatFrontData> = chats_vec.iter().map(move|ChatData { timestamp, msg, sender, was_read:_ }|{
        ChatFrontData { timestamp: DateTime::<Utc>::from_str(timestamp).expect("string is not time").format("%m/%d/%Y @ %H:%M").to_string(), msg: msg.to_owned(), sender: room_id[sender.to_owned()].to_owned() }
    }).collect();

    //query accounts for pfp
    let Ok(Some(pfpurl)) = query_once_option(&data.db, "SELECT * FROM (SELECT page.pfp_url FROM accounts WHERE username=$username).page.pfp_url;", ("username", &opposite)).await else { return RainError::for_js("Error retrieving pfp_url.")};

    let bundle = ChatFrontDataBundle{ data: chats_vec, pfpurl };
    HttpResponse::Ok().json(&bundle)
}










// we must figure out which one serves as the title by finding the one opposite of urself
///This keeps track of identifying the Room ID and who is in it.
pub type RoomID = FixedStrictSetDuo2;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, SurrealValue)]
pub struct FixedStrictSetDuo2{
    inner: [String; 2],
}
impl FixedStrictSetDuo2{
    pub fn create(mut elements: [String; 2]) -> Self{
        elements.sort();
        Self { inner: elements }
    }


    pub fn access_opposite(&self, element: &str) -> Option<String>{
        if self[true] == element {
            Some(self[false].clone())
        } else if self[false] == element{
            Some(self[true].clone())
        }
        else{
            None
        }
    }
}
impl std::ops::Index<bool> for FixedStrictSetDuo2{
    type Output = String;

    fn index(&self, index: bool) -> &Self::Output {
        let index = if index {1} else {0};
        &self.inner[index]
    }
}
impl std::ops::IndexMut<bool> for FixedStrictSetDuo2{
    fn index_mut(&mut self, index: bool) -> &mut Self::Output {
        let index = if index {1} else {0};
        &mut self.inner[index]
    }
}
impl IntoIterator for FixedStrictSetDuo2{
    type Item = String;

    type IntoIter = std::array::IntoIter<String, 2>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}
impl std::ops::Deref for FixedStrictSetDuo2{
    type Target = [String; 2];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}












#[get("/chat")]
pub async fn chat_nav(identity: Option<Identity>) -> impl Responder{
    if identity.is_none(){
        return HttpResponse::Ok().body(NOLOG);
    }
    HttpResponse::Ok().body(CHATNAV)
}

#[derive(serde::Serialize, Debug)]
struct NavLink{
    // url: String, // doesn't need to be transmitted
    room_name: String,
}

#[post("/nav-links")]
pub async fn nav_links(identity: Option<Identity>, data: Data<AppData>) -> impl Responder{
    let Ok(username) = unwrap_identity(identity) else { return RainError::for_js("Identity could not be extracted.")};


    let Ok(rooms) = query_once::<Room>(&data.db, "SELECT * FROM chats WHERE room_id.inner CONTAINS $name;", ("name", &username)).await else{ return r::for_js("Error selecting chats.")};
    let links: Vec<NavLink> = rooms.into_iter().map(|elem|{
        NavLink { room_name: elem.room_id.access_opposite(&username).unwrap_or_default() }
    }).collect();
    // println!("{links:?}");

    HttpResponse::Ok().json(links)
}










#[get("/usr/chats/{uuid}/{n}.png")]
pub async fn chats_access(identity: Option<Identity>, uuidn: Path<(String, String)>, data: Data<AppData>, req: HttpRequest) -> impl Responder{
    let user = match unwrap_identity(identity){
        Ok(r) => r,
        Err(_) => return HttpResponse::NotFound().finish(),
    };

    let (uuid, n) = uuidn.into_inner();

    let Ok(o) = query_once_option::<RoomID>(&data.db, "SELECT * FROM (SELECT room_id FROM chats WHERE id=type::record(\"chats\", $id)).room_id;", ("id", &uuid)).await else { return RainError::for_html_stderr()};
    // if let Err(e) = query_once_option::<RoomID>(&mut db, "SELECT * FROM (SELECT room_id FROM chats WHERE id=type::thing(\"chats\", $id)).room_id;", ("id", &uuid)).await { println!("{}: {e}", line!()); return RainError::for_html(e)};
    // let o: Option<RoomID> = todo!();
    let Some(room) = o else { return RainError::for_html(NOUSER)};
    if !room.contains(&user){
        return RainError::for_html(NOUSER)
    }

    let path = format!("./tmp/chats/{uuid}/{n}.png");



    match NamedFile::open(path){
        Ok(f) => {println!("You're kidding me, right?"); f.into_response(&req)},
        Err(e) => {println!("You-you're kidding me! {e}"); HttpResponse::NotFound().finish()},
    }
}
// struct OCForm{opposite_chatter: String}

#[post("/pics-chats/{opposite_chatter}")]
pub async fn pics_chats(form: MultipartForm<crate::img::ImageUploads>, identity: Option<Identity>, opposite_chatter: Path<String>, data: Data<AppData>) -> impl Responder{
    let Ok(username) = unwrap_identity(identity) else { return r::for_js("Identity failure.")};
    // println!("Tree");
    let room_id = RoomID::create([username, opposite_chatter.into_inner()]);
    let Ok(v) = query_once_option::<String>(&data.db, "SELECT * FROM (SELECT meta::id(id) as a FROM chats WHERE room_id=$room_id)[0].a;", ("room_id", room_id)).await else { return RainError::for_js("Error querying!")};
    let Some(uuid) = v else { return RainError::for_js_user("Chat room does not exist!")};

    let mut file_count = 0;

    if let Ok(paths) = std::fs::read_dir(format!("./tmp/chats/{uuid}/")){
        for path in paths {
            let Ok(path) = path else { break };
            let path = path.path();
            if path.is_file() {
                file_count += 1;
            }
        }
    }

    if file_count >= 10 {
        return r::for_js_user("Chat image limit reached for this room.");
    }

    let mut yourlinks = Vec::new();
    let images = form.into_inner().images;
    for (n, file) in images.into_iter().enumerate() {

        if let Err(e) = verify_img(&file) { return RainError::for_js_user(e)};
        let n = n + file_count;
        let path = format!("./tmp/chats/{uuid}/{n}.png");
        if let Err(e) = upload_file(file, &path).await { return RainError::for_js_user(e)};

        yourlinks.push(format!("/usr/chats/{uuid}/{n}.png"))
    }
    //^^ this may become useful IF we want to prefill the client's text box with the URL.


    HttpResponse::Ok().json(yourlinks)
}










// use actix_web_lab::sse;
// use tokio::sync::mpsc;
// use std::time::Duration;

// #[get("/chat-updates/{opposite}")]
// pub async fn updates(data: Data<AppData>, opposite: Path<String>, self_: Option<Identity>) -> impl Responder {
//     let (tx, rx) = mpsc::channel(10);
//     //maybe make a timer that disables after a certain time bcs this could be intensive?
//     println!("I think I see you!!!!!");
//     let self_ = match unwrap_identity(self_){
//         Ok(i) => i,
//         Err(e) => {println!("IDENTITY ERROR {e}"); return sse::Sse::from_infallible_receiver(rx).with_retry_duration(Duration::from_secs(10));},
//     };
//     let opposite = opposite.into_inner();
//     // let query = "SELECT chats[was_read=false] FROM chats WHERE room_id=$room_id;";
//     let room_id = RoomID::create([opposite, self_]);
//     let mut db = data.db.lock().await;
//     let id = match query_once_option::<String>(&mut db, "SELECT * FROM (SELECT meta::id(id) as a FROM chats WHERE room_id=$room_id)[0].a;", ("room_id", room_id)).await{
//         Ok(Some(o)) => o,
//         Ok(None) => panic!("SSE None Error."),
//         Err(e) => {
//             panic!("SSE Error:{e}");
//         }
//     };
//     drop(db);

//     actix_web::rt::spawn(async move {
//         let db = data.db.lock().await;
//         // Listen to updates on a specific record
//         let mut stream = db.select(("chats", id)).live().await.unwrap();
//         // The returned stream implements `futures::Stream` so we can
//         // use it with `futures::StreamExt`, for example.
//         while let Some(result) = stream.next().await {
//             let result: surrealdb::Notification<Room> = result.unwrap();
//             if tx.send(sse::Event::Data(sse::Data::new("UPDATE"))).await.is_err(){
//                 println!("CLIENT DISCONNECT ERROR");
//                 break;
//             }
//         }
//     });

//     sse::Sse::from_infallible_receiver(rx).with_retry_duration(Duration::from_secs(10))
// }


// use actix::prelude::*;
// use actix_web::{web, Error};
// use actix_web_actors::ws;

// #[get("/chat/updates")]
// async fn websocket_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
//     ws::start(
//        WsSession {},
//        &req,
//        stream,
//     )
// }

// struct WsSession;

// impl Actor for WsSession {
//    type Context = ws::WebsocketContext<Self>;
// }

// impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
//    fn handle(
//        &mut self,
//        msg: Result<ws::Message, ws::ProtocolError>,
//        ctx: &mut Self::Context,
//    ) {
//        match msg {
//            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
//            Ok(ws::Message::Text(text)) => ctx.text(text),
//            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
//         //    Ok(ws::Message)
//            _ => (),
//        }
//    }
// }
