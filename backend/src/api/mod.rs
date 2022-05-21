use actix_web::web;
mod message;
mod point;

pub fn routes() -> actix_web::Scope {
    web::scope("/api")
        .service(message::route)
        .service(point::route)
}
