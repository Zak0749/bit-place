use actix::{Actor, StreamHandler};
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::{ws, ws::Message::Text};

pub struct Session;

impl Actor for Session {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Session {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(Text(text)) = msg {
            println!("Text: {}", text);
            ctx.text(text);
        }
    }
}

#[get("")]
pub async fn route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(Session {}, &req, stream)
}
