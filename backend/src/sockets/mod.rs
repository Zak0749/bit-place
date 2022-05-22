use actix::Addr;
use actix_web::{
    get,
    web::{self, Data, Payload},
    Error, HttpRequest, HttpResponse, Scope,
};
use actix_web_actors::ws;
mod server;
mod session;
use session::Session;

pub use server::Server;

#[get("")]
async fn get_socket(
    req: HttpRequest,
    stream: Payload,
    server: Data<Addr<Server>>,
) -> Result<HttpResponse, Error> {
    ws::start(Session::new(server.get_ref().clone()), &req, stream)
}

pub fn routes() -> Scope {
    web::scope("/ws").service(get_socket)
}
