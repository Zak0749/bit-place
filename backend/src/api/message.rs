use actix::prelude::*;
use actix_redis::{Command, RedisActor, RespValue};
use actix_web::{error, get, web::Data, HttpResponse, Result};
use redis_async::resp_array;

#[get("/message")]
pub async fn route(redis: Data<Addr<RedisActor>>) -> Result<HttpResponse> {
    let data = redis
        .send(Command(resp_array!["GET", "name".to_string()]))
        .await
        .map_err(error::ErrorInternalServerError)?
        .map_err(error::ErrorInternalServerError)?;

    if let RespValue::BulkString(result) = data {
        Ok(HttpResponse::Ok().body(result))
    } else {
        println!("error: {:?}", data);
        Ok(HttpResponse::InternalServerError().body("bad type"))
    }
}
