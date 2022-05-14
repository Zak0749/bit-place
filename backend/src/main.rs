use actix_files::{Files, NamedFile};
use actix_redis::RedisActor;
use actix_web::{get, middleware::Logger, web::Data, App, HttpServer, Responder, Result};
use backend::api;
use backend::sockets;
use std::path::PathBuf;

#[get("/")]
async fn index() -> Result<impl Responder> {
    Ok(NamedFile::open(
        "/usr/local/public/index.html".parse::<PathBuf>().unwrap(),
    )?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let redis_addr = RedisActor::start("db:6379");

        println!("server status up db-con: {}", redis_addr.connected());
        App::new()
            .app_data(Data::new(redis_addr))
            .service(index)
            .service(sockets::routes())
            .service(api::routes())
            .service(Files::new("/", "/usr/local/public/"))
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
