use actix_web::{HttpRequest, HttpResponse, Error, web};
use futures_util::StreamExt;
use crate::server::StreamServer;
use tokio::task;

/// Handle WebSocket connections for chat and stream control
pub async fn chat_ws(
  req: HttpRequest,
  stream: web::Payload,
  server: web::Data<StreamServer>,
) -> Result<HttpResponse, Error> {
  let (response, mut session, mut msg_stream) = actix_ws::handle(&req, stream)?;

  // Process incoming WebSocket messages (chat and commands)
  task::spawn_local(async move {
      while let Some(Ok(msg)) = msg_stream.next().await {
          match msg {
              actix_ws::Message::Text(text) => {
                  log::info!("Received chat message: {}", text);
                  // Here you can process chat messages or commands like `/start` or `/stop`
              }
              actix_ws::Message::Close(_) => {
                  log::info!("Connection closed");
                  break;
              }
              _ => {}
          }
      }
  });

  Ok(response)
}