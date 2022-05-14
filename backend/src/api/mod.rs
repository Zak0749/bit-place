use actix_web::web;
mod message;

pub fn routes() -> actix_web::Scope {
    web::scope("/api").service(message::route)
}
