use actix_identity::Identity;
use actix_session::Session;
use actix_web::{get, post, Responder, HttpResponse, web::{Data, Json, Path}, App, };
use chrono::{DateTime, Utc};
use crate::{db::query, AppData}; 

// use crate::db::Db;
// use actix_sse::SseEvent;
// use actix_sse::SseEvent;
use super::sites::CHAT;

//First, you can use the SortedVec data structure or SortedArray for simple storage or some sort of fixed HashSet.
//Second, we can use a variable: was_seen/was_read/read as a boolean and use it to find which were read and which weren't and by who and which sender.

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
    ///Here, the was_read condition is pertaining to the person opposite of the sender.
    was_read: bool,
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
    let sender = sender == room_id.inner.inner[1]; //if the sender equals the room ids second index, it returns true as chosen before; otherwise it returns false correctly. 

    //get room from db and verify WHO sender is.
    //also edit the room to include this message when logging
    //add the msg data to the room.

    // let mut db = app.db.lock().await;
    // let res2 = query::<Room>(&mut db, "SELECT * FROM accounts WHERE username = type::string($username);", Some(("username", &username))).await.unwrap();
    // let result = res2.get(0).unwrap().as_ref().unwrap();

    let to_database = ChatData{timestamp, msg, sender, was_read: false};
    //log in db
    //TO DATABASE:
    //sender, msg, time sent


    HttpResponse::Ok().body("Successfully logged!")
}

///This keeps track of identifying the Room ID and who is in it.
struct RoomID{
    inner: FixedStrictSetDuo2, // we must figure out which one serves as the title by finding the one opposite of urself
}
impl RoomID{
    fn create(str1: &str, str2: &str) -> RoomID{
        let idx = [str1.min(str2).to_owned(), str1.max(str2).to_owned()];
        RoomID { inner: FixedStrictSetDuo2{inner: idx} }
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
    //read all the ones marked as unread
    //and mark all the read ones as read so they aren't selected anymore
    //do you want it gone on refresh?
    
    let chats_vec : Vec<ChatFrontData> = chats_vec.into_iter().map(move|ChatData { timestamp, msg, sender, was_read:_ }|{
        ChatFrontData { timestamp, msg, sender: room_id.inner[sender].to_owned() }
    }).collect();
    serde_json::to_string(&chats_vec)
}

















///Returns an Ok(()) if all behavior is OK. Returns Err(T) if the object to replace cannot be found.
type ReplaceError<T> = Result<(), T>;


struct FixedStrictSetDuo2{
    inner: [String; 2],
}
impl FixedStrictSetDuo2{
    fn create(mut elements: [String; 2]) -> Self{
        elements.sort();
        Self { inner: elements }
    }
    fn create_from_current_config(mut elements: [String; 2]) -> Self{
        Self { inner: elements }
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

