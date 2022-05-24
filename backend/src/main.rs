use actix_files::{Files, NamedFile};
use actix_redis::RedisActor;
use actix_web::{get, middleware::Logger, web::Data, App, HttpServer, Responder};
use backend::api;
use backend::sockets::{self, Server};
use dotenv::dotenv;
use std::env;

#[get("/")]
async fn index() -> impl Responder {
    NamedFile::open(format!(
        "{}/index.html",
        env::var("PUBLIC_DIR").expect("PUBLIC_DIR not set d:/usr/local/public")
    ))
    .unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let redis = RedisActor::start(env::var("REDIS_URL").expect("REDIS_URL must be set d:db:6379"));
    let server = Server::new(redis.clone());
    let public_dir = env::var("PUBLIC_DIR").expect("PUBLIC_DIR must be set d:/usr/local/public/");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(redis.clone()))
            .app_data(Data::new(server.clone()))
            .service(index)
            .service(sockets::routes())
            .service(api::routes())
            .service(Files::new("/", public_dir.clone()))
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
