//Websockets w/ Actix
//Rustls
//XMPP(S) transfer protocol for messages? Tokio!
//Basically anything that can send and receive messages

//Note: we added Rippedplushie who now has the codebase as of the previous commit.

use actix_session::Session;
use actix_web::{get, post, Responder, HttpResponse, web::{Data, Json, Path}, };
use chrono::{DateTime, Utc};
use crate::db::query;

// use crate::db::Db;
// use actix_sse::SseEvent;
// use actix_sse::SseEvent;
use super::sites::CHAT;
//start using actix-identity for session store additionally
//there was an issue with SurrealSessionStore that must get ironed out before we can proceed

///This represents a chat room with a bunch of chats.
struct Room{
    room: RoomID,
    chats: Vec<ChatData>,
}

/**
This is where the user goes to enter the chats.
The frontend part will consist of a field that is a forum that posts to `/chat/send` with NO redirection.
Upon refresh, all the chats will stay because the chat messages will be added.
*/
#[get("/chats/{receiver}")]
pub async fn chats(receiver: Path<String>, session: actix_session::Session, data: Data<crate::AppData>) -> impl Responder{
    // let sender = super::signup:

    //build a room and send to db. 

    //update it with the most recent chat messages (while having JavaScript add any new ones to the DOM)

    HttpResponse::Ok().body(CHAT)
}

///The chat data received from the frontend.
struct Chat{
    // time: DateTime<Utc>, 
    // sender: String, 
    receiver: String,
    msg: String,
}

///The chat data stored in the database.
struct ChatData{
    time: DateTime<Utc>,
    msg: String,
    sender: bool, //if true, it was sent by me, if false, it was sent by my chatter
}

///The chat data given to the frontend.
#[derive(serde::Serialize)]
struct ChatFrontData{
    time: String,
    msg: String,
    sender: String,
}


//text_message
/// This, given the msg and the sender, sends a message and logs it in the Database.
/// It then can be retrieved from the receive message function.
#[post("/chat/send")]
pub async fn send(msg: Json<String>, session: Session, app: Data<crate::AppData>) -> impl Responder{

    let sender = super::signup::retrieve_user(session).unwrap().unwrap();
    let time = Utc::now();
    let msg = msg.into_inner();
    //get room from db and verify WHO sender is.
    //also edit the room to include this message when logging

    // let mut db = app.db.lock().await;
    // let res2 = query::<Room>(&mut db, "SELECT * FROM accounts WHERE username = type::string($username);", Some(("username", &username))).await.unwrap();
    // let result = res2.get(0).unwrap().as_ref().unwrap();


    let sender = true;

    let to_database = ChatData{time, msg, sender};
    //log in db
    //TO DATABASE:
    //sender, msg, time sent
    "Successfully logged!"
}

///This keeps track of identifying the Room ID.
struct RoomID{
    opposite: String, //this serves as the title
    same: String,
    //problem: sender and receiver may have a better way to represent this.
    //hash set? sorted array of 2 items?
}

//GIVE THE FRONTEND : Vec<ChatFrontData>
/// This uses long polling to eventually give the frontend a Vec<ChatFrontData> which is useful for adding it to the DOM.
/// The difference here is that, now, we will use the LIVE feature on SurrealDB to find the next update and use it. 
/// This is opposed to the method used above when refreshing the page which simply obtains all of the Vec<ChatFrontData> rather than the new ones.
#[post("/chat/receive")]
pub async fn receive(session: Session, opposite: Json<String>) -> impl Responder{
    //which room receiving from? tell me in js
    let same = super::signup::retrieve_user(session).unwrap().unwrap();
    let opposite = opposite.into_inner();
    let room_id = RoomID{same, opposite};
    //get room_id from db and return new messages with the LIVE query.



    let vec : Vec<ChatFrontData> = Vec::new();
    //change vec to be the vec of chatfront data which is what the DOM needs
    serde_json::to_string(&vec)
}



