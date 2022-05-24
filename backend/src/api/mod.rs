use actix_web::web;
mod grid;
mod point;

pub fn routes() -> actix_web::Scope {
    web::scope("/api")
        .service(point::route)
        .service(grid::route)
}
