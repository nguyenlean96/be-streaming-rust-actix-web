use actix::prelude::*;
use actix_ws::ws;

pub struct ChatSession {
  hb: Instant, // Heartbeat for detecting client activity
}

impl Actor for ChatSession {
  type Context = ws::WebsocketContext<Self>;

  fn started(&mut self, ctx: &mut Self::Context) {
      self.hb(ctx);  // Start heartbeat for connection
  }
}

impl ChatSession {
  fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
      ctx.run_interval(Duration::from_secs(5), |act, ctx| {
          if Instant::now().duration_since(act.hb) > Duration::from_secs(10) {
              ctx.stop();  // Disconnect if no heartbeat
              return;
          }
          ctx.ping(b"ping"); // Send ping to keep connection alive
      });
  }
}

// Handle incoming messages from clients
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
  fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
      match msg {
          Ok(ws::Message::Text(text)) => ctx.text(text), // Echo back messages
          _ => (),
      }
  }
}