use actix_files::{Files, NamedFile};
use actix_redis::RedisActor;
use actix_web::{get, middleware::Logger, web::Data, App, HttpServer, Responder};
use backend::api;
use backend::sockets;
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

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(RedisActor::start(
                env::var("REDIS_URL").expect("REDIS_URL must be set d:db:6379"),
            )))
            .service(index)
            .service(sockets::routes())
            .service(api::routes())
            .service(Files::new(
                "/",
                env::var("PUBLIC_DIR").expect("PUBLIC_DIR must be set d:/usr/local/public/"),
            ))
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
