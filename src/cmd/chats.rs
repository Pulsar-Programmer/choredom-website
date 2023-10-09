use actix_identity::Identity;
use actix_session::Session;
use actix_web::{get, post, Responder, HttpResponse, web::{Data, Json, Path}, App, };
use chrono::{DateTime, Utc};
use crate::{db::query, AppData}; 

// use crate::db::Db;
// use actix_sse::SseEvent;
// use actix_sse::SseEvent;
use super::sites::CHAT;


///This represents a chat room with a bunch of chats.
struct Room{
    room_id: RoomID,
    chats: Vec<ChatData>,
}

/**
This is where the user goes to enter the chats.
The frontend part will consist of a field that is a forum that posts to `/chat/send` with NO redirection.
Upon refresh, all the chats will stay because the chat messages will be added.
*/
#[get("/chats/{receiver}")]
pub async fn chats(receiver: Path<String>, identity: Option<Identity>, data: Data<crate::AppData>) -> impl Responder{
    let sender = super::signup::retrieve_user(identity.unwrap()).unwrap();
    let receiver = receiver.into_inner();
    let room_id = RoomID::create(&sender, &receiver);

    //build a room and send to db if one doesn't exist
    let one_exists = false; //how do we check if one exists with the database as a shortcut?
    if !one_exists{
        let chats = Vec::new();
        let room = Room{room_id, chats};
        //send to db now
    }

    //get the most recent chat messages from surrealdb
    //update the DOM the most recent chat messages (later having JavaScript add any new ones to the DOM)
    let html = CHAT; 


    HttpResponse::Ok().body(html)
}

///The chat data stored in the database.
struct ChatData{
    timestamp: DateTime<Utc>,
    msg: String,
    ///Be careful here, as we must ensure sender is in the room, as in, it is contained within the `RoomID`.
    ///There was a thought of using a boolean here to save storage, but we decided not to integrate it.
    ///Nevermind we are changing this again to a boolean to save immense storage. I am a monkie. Sorry for that.
    sender: bool, 
}

///The chat data given to the frontend.
#[derive(serde::Serialize)]
struct ChatFrontData{
    timestamp: DateTime<Utc>,
    msg: String,
    sender: String, 
}

//text_message
/// This, given the msg and the sender, sends a message and logs it in the Database.
/// It then can be retrieved from the receive message function.
#[post("/chat/send")]
pub async fn send(json: Json<(String, String)>, identity: Option<Identity>, app: Data<crate::AppData>) -> impl Responder{
    //Here, `json` represents the reciever and the msg intended to be sent.
    let sender = super::signup::retrieve_user(identity.unwrap()).unwrap();
    let timestamp = Utc::now();
    let (room_title, msg) = json.into_inner();
    let room_id = RoomID::create(&room_title, &sender);
    let sender = sender == room_id.inner[1]; //if the sender equals the room ids second index, it returns true as chosen before; otherwise it returns false correctly. 

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

///This keeps track of identifying the Room ID and who is in it.
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
pub async fn receive(identity: Option<Identity>, opposite: Json<String>, data: Data<AppData>) -> impl Responder{
    let same = super::signup::retrieve_user(identity.unwrap()).unwrap();
    let opposite = opposite.into_inner();
    let room_id = RoomID::create(&same, &opposite);
    //select with room_id from db and return new messages with the LIVE query.
    let db = data.db.lock().await;
    // let db = surrealdb::kvs::Datastore::new("memory").await.unwrap().with_capabilities(surrealdb::dbs::Capabilities::all());

    
    let chats_vec : Vec<ChatData> = Vec::new(); 
    // get the newly LIVE query ones and sleep or something until this is given.
    
    let chats_vec : Vec<ChatFrontData> = chats_vec.into_iter().map(move|ChatData { timestamp, msg, sender }|{
        ChatFrontData { timestamp, msg, sender: room_id.inner[if sender {1} else {0}].clone() }
    }).collect();
    serde_json::to_string(&chats_vec)
}



