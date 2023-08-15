//SET UP A WEBSOCKET SERVER HERE LATER

// #[get("/send-message")]
// async fn send_message() -> impl Responder{
//     todo!() as HttpResponse
// }


// #[post("/receive_messages")]
// async fn receive_messages() -> impl Responder{
//     todo!() as HttpResponse
// }

pub mod chat;
mod message;
mod server;
mod session;