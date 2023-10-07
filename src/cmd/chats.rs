use actix_session::Session;
use actix_web::{get, post, Responder, HttpResponse, web::{Data, Json, Path}, };
use chrono::{DateTime, Utc};
use crate::db::query; 

// use crate::db::Db;
// use actix_sse::SseEvent;
// use actix_sse::SseEvent;
use super::sites::CHAT;


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

    //build a room and send to db if one doesn't exist

    //get the most recent chat messages from surrealdb
    //update it with the most recent chat messages (while having JavaScript add any new ones to the DOM)

    HttpResponse::Ok().body(CHAT)
}

///The chat data stored in the database.
///The chat data given to the frontend, replacing ChatFrontData from before.
#[derive(serde::Serialize)]
struct ChatData{
    timestamp: DateTime<Utc>,
    msg: String,
    ///Be careful here, as we must ensure sender is in the room, as in, it is contained within the `RoomID`.
    sender: String, 
}

//text_message
/// This, given the msg and the sender, sends a message and logs it in the Database.
/// It then can be retrieved from the receive message function.
#[post("/chat/send")]
pub async fn send(json: Json<(String, String)>, session: Session, app: Data<crate::AppData>) -> impl Responder{
    //Here, `json` represents the reciever and the msg intended to be sent.
    let sender = super::signup::retrieve_user(session).unwrap().unwrap();
    let timestamp = Utc::now();
    let (room_title, msg) = json.into_inner();
    let room_id = RoomID::create(&room_title, &sender);

    //get room from db and verify WHO sender is.
    //also edit the room to include this message when logging
    //add the msg data to the room.

    // let mut db = app.db.lock().await;
    // let res2 = query::<Room>(&mut db, "SELECT * FROM accounts WHERE username = type::string($username);", Some(("username", &username))).await.unwrap();
    // let result = res2.get(0).unwrap().as_ref().unwrap();

    let to_database = ChatData{timestamp, msg, sender};
    //log in db
    //TO DATABASE:
    //sender, msg, time sent


    HttpResponse::Ok().body("Successfully logged!")
}

///This keeps track of identifying the Room ID.
struct RoomID{
    inner: [String; 2], // we must figure out which one serves as the title by finding the one opposite of urself
}
impl RoomID{
    fn create(str1: &str, str2: &str) -> RoomID{
        let idx = [str1.min(str2).to_owned(), str1.max(str2).to_owned()];
        RoomID { inner: idx }
    }
}

//GIVE THE FRONTEND : Vec<ChatFrontData>
/// This uses long polling to eventually give the frontend a Vec<ChatFrontData> which is useful for adding it to the DOM.
/// The difference here is that, now, we will use the LIVE feature on SurrealDB to find the next update and use it. 
/// This is opposed to the method used above when refreshing the page which simply obtains all of the Vec<ChatFrontData> rather than the new ones.
#[post("/chat/receive")]
pub async fn receive(session: Session, opposite: Json<String>) -> impl Responder{
    let same = super::signup::retrieve_user(session).unwrap().unwrap();
    let opposite = opposite.into_inner();
    let room_id = RoomID::create(&same, &opposite);
    //select with room_id from db and return new messages with the LIVE query.

    let vec : Vec<ChatData> = Vec::new();
    //change vec to be the vec of chatfront data which is what the DOM needs
    serde_json::to_string(&vec)
}



