use actix_files::{Files, NamedFile};
use actix_redis::RedisActor;
use actix_web::{get, middleware::Logger, web::Data, App, HttpServer, Responder};
use backend::api;
use backend::sockets;

#[get("/")]
async fn index() -> impl Responder {
    NamedFile::open("/usr/local/public/index.html").unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(RedisActor::start("db:6379")))
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
