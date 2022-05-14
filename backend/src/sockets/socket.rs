use actix::{Actor, StreamHandler};
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::{ws, ws::Message::Text};

pub struct Socket;

impl Actor for Socket {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Socket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(Text(text)) = msg {
            println!("Text: {}", text);
            ctx.text(text);
        }
    }
}

#[get("")]
// pub async fn route() -> impl Responder {
pub async fn route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(Socket {}, &req, stream)
}
