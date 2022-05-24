use actix::Addr;
use actix_redis::{Command, RedisActor, RespValue};
use actix_web::{error, get, web, web::Data, HttpResponse, Result};
use redis_async::resp_array;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

#[get("/point/{x}/{y}")]
pub async fn route(redis: Data<Addr<RedisActor>>, point: web::Path<Point>) -> Result<HttpResponse> {
    let data = redis
        .send(Command(resp_array![
            "GET",
            format!("{}:{}", point.x, point.y)
        ]))
        .await
        .map_err(error::ErrorInternalServerError)?
        .map_err(error::ErrorInternalServerError)?;

    if let RespValue::BulkString(result) = data {
        Ok(HttpResponse::Ok().body(result))
    } else {
        println!("error: {:?}", data);
        Ok(HttpResponse::InternalServerError().body("null"))
    }
}
