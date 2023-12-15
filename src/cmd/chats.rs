use actix_identity::Identity;
use actix_web::{get, post, Responder, HttpResponse, web::{Data, Json, Path}};
use chrono::{DateTime, Utc};
use crate::{db::{query_once, sole_query}, AppData, cmd::sites::NOLOG, RainError}; 
use super::sites::{CHAT, CHATNAV, NOUSER};
use super::signup::unwrap_identity;
use RainError as r;

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
pub async fn chats_get(receiver: Path<String>, app_data: Data<AppData>, identity: Option<Identity>) -> impl Responder{
    let receiver = receiver.into_inner();
    let mut db = app_data.db.lock().await;
    let Ok(result) = query_once::<super::signup::Account>(&mut db, "SELECT * FROM accounts WHERE username=$username;", ("username", &receiver)).await else {return RainError::for_html_stderr()};
    if result.len() != 1 {
        return RainError::for_html(NOUSER);
    }
    let Ok(sender) = unwrap_identity(identity) else {return RainError::for_html(NOLOG)};
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

#[post("/chats_obtain")]
pub async fn chats_obtain(receiver: Json<String>, identity: Option<Identity>, data: Data<crate::AppData>) -> impl Responder{
    let Ok(sender) = unwrap_identity(identity) else { return RainError::for_js("Identity cannot be extracted.")};
    let receiver = receiver.into_inner();
    let room_id = RoomID::create([sender, receiver.clone()]);

    //build a room and send to db if one doesn't exist >> use indicies for this
    let mut db = data.db.lock().await;


    let opposite = room_id[true] == receiver;
    let useful_data = ChatDBQuery{ sender: opposite, room_id: room_id.clone() };

    // we must first redeem them as all read, since you are entering
    let Ok(_) = sole_query(&mut db, "UPDATE chats SET messages[WHERE was_read = false AND sender = $sender].was_read = true WHERE room_id = $room_id;", &useful_data).await else {return r::for_js("Error updating.")};
    let Ok(result) = query_once::<Room>(&mut db, "SELECT * FROM chats WHERE room_id = $room_id;", ("room_id", &room_id)).await else { return r::for_js("Error getting chats.")};
    let Some(result) = result.get(0) else {
        let room = Room{room_id, messages: Vec::new()};
        let Ok(_) = sole_query(&mut db, "CREATE chats SET room_id=$room_id, chats=$chats;", room).await else { return r::for_js("Error creating new chat room.")};
        return HttpResponse::Ok().json(&Vec::<ChatData>::new());
    };
    let Room { room_id: _, messages: vec } = result;

    let vec : Vec<ChatFrontData> = vec.iter().map(move|ChatData { timestamp, msg, sender, was_read:_ }|{
        ChatFrontData { timestamp: timestamp.format("%m/%d/%Y").to_string(), msg: msg.to_owned(), sender: room_id[sender.to_owned()].to_owned() }
    }).collect();

    //update the DOM the most recent chat messages (later having JavaScript add any new ones to the DOM)
    HttpResponse::Ok().json(vec)
}












///The chat data stored in the database.
#[derive(serde::Serialize, Debug, serde::Deserialize, Clone)]
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

    let to_database = ChatData{timestamp, msg: msg.clone(), sender, was_read: false};
    let fake_room = FakeRoom{chat: to_database, room_id};

    let mut db = app.db.lock().await;
    let Ok(..) = sole_query(&mut db, "UPDATE chats SET messages += $chat WHERE room_id = $room_id;", fake_room).await else { return r::for_js("Error adding chat to room.")};
    
    let to_frontend = ChatFrontData{ timestamp: timestamp.format("%m/%d/%Y").to_string(), msg, sender: named_sender };
    // println!("Chat bounceback: {to_frontend:?}");
    HttpResponse::Ok().json(to_frontend)
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
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
    let mut db = data.db.lock().await;
    println!("{useful_data:?}");
    let Ok(res) = query_once::<ChatDBGiven>(&mut db, "SELECT messages[WHERE was_read = false AND sender = $sender] FROM chats WHERE room_id = $room_id;", &useful_data).await else{ return r::for_js("Could not select chats.")};
    let Some(dbgiven) = res.get(0) else {return HttpResponse::Ok().json(Vec::<ChatDBGiven>::new())};
    let chats_vec = &dbgiven.messages;
    //mark as read right before
    let Ok(..) = sole_query(&mut db, "UPDATE chats SET messages[WHERE was_read = false AND sender = $sender].was_read = true WHERE room_id = $room_id;", &useful_data).await else { return r::for_js("Could not mark chats as read.")};

    let chats_vec : Vec<ChatFrontData> = chats_vec.iter().map(move|ChatData { timestamp, msg, sender, was_read:_ }|{
        ChatFrontData { timestamp: timestamp.format("%m/%d/%Y").to_string(), msg: msg.to_owned(), sender: room_id[sender.to_owned()].to_owned() }
    }).collect();
    HttpResponse::Ok().json(&chats_vec)
}










// we must figure out which one serves as the title by finding the one opposite of urself
///This keeps track of identifying the Room ID and who is in it.
pub type RoomID = FixedStrictSetDuo2;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
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


    let mut db = data.db.lock().await;
    let Ok(rooms) = query_once::<Room>(&mut db, "SELECT * FROM chats WHERE room_id.inner CONTAINS $name;", ("name", &username)).await else{ return r::for_js("Error selecting chats.")};
    let links: Vec<NavLink> = rooms.into_iter().map(|elem|{
        NavLink { room_name: elem.room_id.access_opposite(&username).unwrap_or_default() }
    }).collect();
    // println!("{links:?}");
    
    HttpResponse::Ok().json(links)
}













#[post("/pics-chats")]
pub async fn pics_chats(form: actix_multipart::Multipart, identity: Option<Identity>) -> impl Responder{
    let Ok(username) = unwrap_identity(identity) else { return r::for_js("Identity failure.")};
    println!("Tree");
    crate::img::process_multipart(form, format!("chats/{username}/pics")).await.unwrap();
    //^^ this may become useful IF we want to prefill the client's text box with the URL.
    //^^ we use username whereas uuid is preferred. How do we extract UUID? We would have to convert it to JS (conveluded but.. possible? process)
    HttpResponse::SeeOther().append_header((actix_web::http::header::LOCATION, "/chat")).body(CHATNAV)
}