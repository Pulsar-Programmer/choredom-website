use actix::{Actor, prelude::{Supervised, SystemService}};
use actix_web::{web, Error, HttpRequest, Responder, HttpResponse};
use actix_web_actors::ws;
use chrono::{DateTime, Utc};
use crate::AppData;

#[actix_web::get("/chat")]
async fn chat() -> impl Responder{
    HttpResponse::Ok().body(super::super::sites::CHAT)
}

// #[actix_web::get("/chat/ws")]
// async fn chat_ws(req: HttpRequest, stream: web::Payload) -> Result<impl Responder, Error> {
//     ws::start(CSession::default(), &req, stream)
// }


// #[derive(Clone, actix::prelude::Message)]
// #[rtype(result = "anyhow::Result<()>")]
// enum CSignal{
//     Text(CMessage),
//     // ServerSignals go here
//     ServerError{
//         msg: String,
//     }


    
// }

// struct CMessage{
//     sender: usize,
//     msg: String,
//     timestamp: DateTime<Utc>,
// }
// #[derive(Default)]
// struct CServer{
//     data: actix_web::web::Data<AppData>,
//     sessions: HashMap<String, Recipient<Message>>,
// }
// impl Actor for CServer{
//     type Context = actix::prelude::Context<Self>;
// }
// impl SystemService for CServer {}
// impl Supervised for CServer {}








// struct CSession{
//     id: usize,
//     username: String,
// }
// impl Actor for CSession{
//     type Context = ws::WebsocketContext<Self>;
// }
// impl actix::prelude::StreamHandler<Result<CSignal, ws::ProtocolError>> for CSession{
//     fn handle(&mut self, item: Result<CSignal, ws::ProtocolError>, ctx: &mut Self::Context) -> anyhow::Result<()> {
//         let msg = match item {
//             Err(e) => {
//                 ctx.stop();
//                 return Err(e);
//             }
//             Ok(msg) => msg,
//         };

//         match msg {
//             CSignal::Text(text) => {







//                 Ok(())
//             },
//             CSignal::ServerError { msg } => {
//                 println!("SERVER ERROR : {msg}");
//                 Err(msg)
//             },
//         }
//     }
// }
// ws::Message::Text(text) => {
            //     let msg = text.trim();

            //     if msg.starts_with('/') {
            //         let mut command = msg.splitn(2, ' ');

            //         match command.next() {
            //             Some("/list") => self.list_rooms(ctx),

            //             Some("/join") => {
            //                 if let Some(room_name) = command.next() {
            //                     self.join_room(room_name, ctx);
            //                 } else {
            //                     ctx.text("!!! room name is required");
            //                 }
            //             }

            //             Some("/name") => {
            //                 if let Some(name) = command.next() {
            //                     self.name = Some(name.to_owned());
            //                     ctx.text(format!("name changed to: {name}"));
            //                 } else {
            //                     ctx.text("!!! name is required");
            //                 }
            //             }

            //             _ => ctx.text(format!("!!! unknown command: {msg:?}")),
            //         }

            //         return;
            //     }
            //     self.send_msg(msg);
            // }
            // ws::Message::Close(reason) => {
            //     ctx.close(reason);
            //     ctx.stop();
            // }
            // _ => {}