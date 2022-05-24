use std::collections::HashMap;

use actix::Addr;
use actix_redis::{resp_array, Command, RedisActor, RespValue};
use actix_web::{error, get, web::Data, Result};
use futures::future::join_all;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Hash, Clone, Debug, Deserialize, Serialize)]
struct Point {
    x: i32,
    y: i32,
    color: String,
}

#[get("/grid")]
pub async fn route(redis: Data<Addr<RedisActor>>) -> Result<String> {
    let vals = (0..=100).flat_map(|x| {
        (0..=100)
            .map(|y| Point {
                x,
                y,
                color: "FFFFFF".to_string(),
            })
            .collect::<Vec<Point>>()
    });

    let json = serde_json::to_string(
        &vals
            .clone()
            .zip(
                join_all(vals.map(|point| {
                    redis.send(Command(resp_array![
                        "GET",
                        format!("{}:{}", point.x, point.y)
                    ]))
                }))
                .await
                .iter()
                .map(|v| match v {
                    Ok(Ok(RespValue::BulkString(s))) => {
                        std::str::from_utf8(s).unwrap_or("FFFFFF").to_string()
                    }
                    _ => "FFFFFF".to_string(),
                }),
            )
            .map(|(point, color)| Point {
                x: point.x,
                y: point.y,
                color,
            })
            .collect::<Vec<Point>>(),
    );

    json.map_err(error::ErrorInternalServerError)
}
