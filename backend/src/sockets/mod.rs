use actix_web::{web, Scope};
mod socket;

pub fn routes() -> Scope {
    web::scope("/ws").service(socket::route)
}
