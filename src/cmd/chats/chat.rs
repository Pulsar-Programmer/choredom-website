use actix_web::{web, Error, HttpRequest, Responder, HttpResponse};
use actix_web_actors::ws;
use super::session::WsChatSession;

#[actix_web::get("/chat")]
async fn chat() -> impl Responder{
    HttpResponse::Ok().body(super::super::sites::CHAT)
}

#[actix_web::get("/chat/ws")]
async fn chat_ws(req: HttpRequest, stream: web::Payload) -> Result<impl Responder, Error> {
    ws::start(WsChatSession::default(), &req, stream)
}