//Websockets w/ Actix
//Rustls
//XMPP(S) transfer protocol for messages? Tokio!
//Basically anything that can send and receive messages

//Note: we added Rippedplushie who now has the codebase as of the previous commit.

use actix_web::{get, post, Responder, HttpResponse, web::{Data, Json, Path}, };
use chrono::{DateTime, Utc};
// use crate::db::Db;
// use actix_sse::SseEvent;
// use actix_sse::SseEvent;
use super::sites::CHAT;


struct Room{
    receiver: String, //this serves as the title
    sender: String,
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
struct ChatFrontData{
    time: String,
    msg: String,
    sender: String,
}


//text_message
#[post("/chat/send")]
pub async fn send(msg: Json<String>) -> impl Responder{



    //TO DATABASE:
    //sender, msg, time sent
    HttpResponse::Ok().body("")
}

//GIVE THE FRONTEND : Vec<ChatFrontData>\
/// This uses long polling to eventually give the frontend a Vec<ChatFrontData> which is useful for adding it to the DOM.
/// The difference here is that, now, we will use the LIVE feature on SurrealDB to find the next update and use it. 
/// This is opposed to the method used above when refreshing the page which simply obtains all of the Vec<ChatFrontData> rather than the new ones.
#[post("/chat/receive")]
pub async fn receive() -> impl Responder{






    HttpResponse::Ok().body("")
}



