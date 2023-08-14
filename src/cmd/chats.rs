use actix_web::{web, post, HttpResponse, Responder, get};
use std::collections::HashMap;
use crate::AppData;
use super::sites::CHAT;



//SET UP A WEBSOCKET SERVER HERE LATER








#[get("/send-message")]
async fn send_message() -> impl Responder{
    todo!() as HttpResponse
}


#[post("/receive_messages")]
async fn receive_messages() -> impl Responder{
    todo!() as HttpResponse
}




// #[derive(Default, serde::Deserialize)]
// pub struct ChatClientTransmitter{
//     clients: HashMap<u32, EventualData>, // For simplicity, we'll use a HashMap to store connected clients.
//     msgs: Vec<(u32, String)>, 
// }
// type CCT = ChatClientTransmitter;

// #[derive(serde::Deserialize)]
// struct EventualData;

// #[get("/chats/connect/{client_id}")]
// pub async fn connect(data: web::Data<AppData>, client_id: web::Path<u32>, session: Session) -> impl Responder {
//     let mut cct = todo!();
//     cct.clients.insert(client_id.into_inner(), EventualData);
//     HttpResponse::Ok().finish()
// }

// #[get("/chats/disconnect/{client_id}")]
// pub async fn disconnect(data: web::Data<AppData>, client_id: web::Path<u32>) -> impl Responder {
//     let mut cct: HashMap<_, _> = todo!();
//     cct.remove(&client_id.into_inner());
//     HttpResponse::Ok().finish()
// }

// #[get("/chats/send/{client_id}")]
// pub async fn send_message(data: web::Data<AppData>, client_id: web::Path<u32>, msg: web::Json<String>) -> impl Responder {
//     let mut cct = data.transmitters.cct.lock().await;
//     let client_id = client_id.into_inner();
//     if let Some(data) = cct.clients.get(&client_id) {
//         // Here you would typically broadcast the message to all connected clients.
//         cct.msgs.push((client_id, msg.into_inner()));
//         HttpResponse::Ok().finish()
//     } else {
//         HttpResponse::NotFound().finish()
//     }
// }

// #[get("/chats/receive/")]
// pub async fn receive_message(data: web::Data<AppData>) -> impl Responder {
//     let mut cct = data.transmitters.cct.lock().await;

//     // Here you would typically return any messages that have been sent to the client.
//     serde_json::to_string(&cct.msgs)
// }