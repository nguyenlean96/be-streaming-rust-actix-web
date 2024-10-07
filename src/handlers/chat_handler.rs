use actix_web::{web, HttpRequest, HttpResponse, Error};
use actix_ws::ws;
use crate::actors::chat::ChatSession;

pub async fn chat_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(ChatSession { hb: Instant::now() }, &req, stream)
}