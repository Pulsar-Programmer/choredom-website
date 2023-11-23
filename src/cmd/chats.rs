use actix_identity::Identity;
use actix_session::Session;
use actix_web::{get, post, Responder, HttpResponse, web::{Data, Json, Path}, App, };
use chrono::{DateTime, Utc};
use crate::{db::query, AppData}; 
use super::sites::CHAT;
use super::signup::retrieve_user;

///This represents a chat room with a bunch of chats.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
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
pub async fn chats(receiver: Path<String>, app_data: Data<AppData>, identity: Option<Identity>) -> impl Responder{
    let receiver = receiver.into_inner();
    let mut db = app_data.db.lock().await;
    let res = query::<super::signup::Account>(&mut db, "SELECT * FROM accounts WHERE username=$username;", ("username", &receiver)).await.unwrap();
    let result = res.get(0).unwrap().as_ref().unwrap();
    //^^^ replace this with indices in future
    //vvvv If it is greater we have issues, if it less then this is valid behavior
    if result.len() != 1 {
        //^feh
        return HttpResponse::BadRequest().body("No messaging undefined users!");
    }
    let Some(identity) = identity else {return HttpResponse::BadRequest().body("Log in to access chats.")};
    let sender = super::signup::retrieve_user(identity).unwrap();
    if sender == receiver{
        //^feh
        return HttpResponse::BadRequest().body("No sending chats to yourself!")
    }
    HttpResponse::Ok().body(CHAT)
}

#[post("/chats_obtain")]
pub async fn chats_obtain(receiver: Json<String>, identity: Option<Identity>, data: Data<crate::AppData>) -> impl Responder{
    let sender = super::signup::retrieve_user(identity.unwrap()).unwrap();
    let receiver = receiver.into_inner();
    let room_id = RoomID::create([sender, receiver.clone()]);

    //build a room and send to db if one doesn't exist >> use indicies for this
    let mut db = data.db.lock().await;
    let res2 = query::<Room>(&mut db, "SELECT * FROM chats WHERE room_id = $room_id;", ("room_id", &room_id)).await.unwrap();
    let result = res2.get(0).unwrap().as_ref().unwrap();
    if result.len() != 1{
        let vec_chats = Vec::new();
        let room = Room{room_id, messages: vec_chats};
        query::<()>(&mut db, "CREATE chats SET room_id=$room_id, chats=$chats;", room).await.unwrap();
        return HttpResponse::Ok().json(&Vec::<ChatData>::new());
    }
    let result = result.get(0).unwrap();
    let Room { room_id: _, messages: vec } = result;

    let vec : Vec<ChatFrontData> = vec.iter().map(move|ChatData { timestamp, msg, sender, was_read:_ }|{
        ChatFrontData { timestamp: timestamp.to_owned(), msg: msg.to_owned(), sender: room_id[sender.to_owned()].to_owned() }
    }).collect();

    //update the DOM the most recent chat messages (later having JavaScript add any new ones to the DOM)
    HttpResponse::Ok().json(vec)
}












///The chat data stored in the database.
#[derive(serde::Serialize, Debug, serde::Deserialize)]
pub struct ChatData{
    pub timestamp: DateTime<Utc>,
    pub msg: String,
    ///Be careful here, as we must ensure sender is in the room, as in, it is contained within the `RoomID`.
    ///There was a thought of using a boolean here to save storage, but we decided not to integrate it.
    ///Nevermind we are changing this again to a boolean to save immense storage. I am a monkie. Sorry for that.
    pub sender: bool, 
    ///Here, the was_read condition is pertaining to the person opposite of the sender.
    pub was_read: bool,
}

///The chat data given to the frontend.
#[derive(serde::Serialize)]
struct ChatFrontData{
    timestamp: DateTime<Utc>,
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

//text_message
/// This, given the msg and the sender, sends a message and logs it in the Database.
/// It then can be retrieved from the receive message function.
#[post("/chat/send")]
pub async fn send(json: Json<FrontSentData>, identity: Option<Identity>, app: Data<crate::AppData>) -> impl Responder{
    println!("Chat sent!");
    //Here, `json` represents the reciever and the msg intended to be sent.
    let sender = super::signup::retrieve_user(identity.unwrap()).unwrap();
    let timestamp = Utc::now();
    let FrontSentData{room_title, msg} = json.into_inner();
    let room_id = RoomID::create([room_title, sender.clone()]);
    let sender = &sender == room_id.get(1).unwrap(); //if the sender equals the room ids second index, it returns true as chosen before; otherwise it returns false correctly. 

    let to_database = ChatData{timestamp, msg, sender, was_read: false};
    let fake_room = FakeRoom{chat: to_database, room_id};

    let mut db = app.db.lock().await;
    query::<()>(&mut db, "UPDATE chats SET messages += $chat WHERE room_id = $room_id;", fake_room).await.unwrap();

    HttpResponse::Ok().body("Successfully logged!")
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ChatDBGiven{
    pub messages: Vec<ChatData>,
}


//GIVE THE FRONTEND : Vec<ChatFrontData>
/// This uses long polling to eventually give the frontend a Vec<ChatFrontData> which is useful for adding it to the DOM.
/// The difference here is that, now, we will use the LIVE feature on SurrealDB to find the next update and use it. 
/// This is opposed to the method used above when refreshing the page which simply obtains all of the Vec<ChatFrontData> rather than the new ones.
#[post("/chat/receive")]
pub async fn receive(identity: Option<Identity>, opposite: Json<String>, data: Data<AppData>) -> impl Responder{
    let same = super::signup::retrieve_user(identity.unwrap()).unwrap();
    let opposite = opposite.into_inner();
    let room_id = RoomID::create([same, opposite]);

    //must incorporate the WHILE LET and the EVENT kind of idea to wait for the long polling to end and such and such
    let mut db = data.db.lock().await;
    let res = query::<ChatDBGiven>(&mut db, "SELECT messages[WHERE was_read = false] FROM chats WHERE room_id = $room_id;", ("room_id", &room_id)).await.unwrap();
    let Some(dbgiven) = &res.get(0).unwrap().as_ref().unwrap().get(0) else {return HttpResponse::NoContent().finish()}; //how do you return none for god sake
    let chats_vec = &dbgiven.messages;
    //mark as read right before
    query::<()>(&mut db, "UPDATE chats SET messages[WHERE was_read = false].was_read = true WHERE room_id = $room_id;", ("room_id", &room_id)).await.unwrap();

    let chats_vec : Vec<ChatFrontData> = chats_vec.iter().map(move|ChatData { timestamp, msg, sender, was_read:_ }|{
        ChatFrontData { timestamp: timestamp.to_owned(), msg: msg.to_owned(), sender: room_id[sender.to_owned()].to_owned() }
    }).collect();
    HttpResponse::Ok().json(&chats_vec)
}

















// we must figure out which one serves as the title by finding the one opposite of urself
///This keeps track of identifying the Room ID and who is in it.
pub type RoomID = FixedStrictSetDuo2;
#[derive(serde::Serialize, serde::Deserialize, Debug)]
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












#[get("/chat-nav")]
pub async fn chat_nav() -> impl Responder{
    HttpResponse::Ok().body(CHAT)
}

#[derive(serde::Serialize)]
struct NavLink{
    // url: String, // doesn't need to be transmitted
    room_name: String,
}

#[post("/nav-links")]
pub async fn nav_links(identity: Option<Identity>, data: Data<AppData>) -> impl Responder{
    let username = retrieve_user(identity.unwrap()).unwrap();


    let mut db = data.db.lock().await;
    let rooms = query::<Room>(&mut db, "SELECT * FROM chats WHERE room_id CONTAINS $name;", ("name", &username)).await.unwrap();
    let rooms = rooms.get(0).unwrap().as_ref().unwrap();
    let links: Vec<NavLink> = rooms.into_iter().map(|elem|{
        NavLink { room_name: elem.room_id.access_opposite(&username).unwrap() }
    }).collect();

    
    HttpResponse::Ok().json(links)
}







