use actix::{Actor, Addr, AsyncContext, Handler, Message, StreamHandler};
use actix_redis::{Command, RedisActor};
use actix_web::{
    get,
    web::{self, Data},
    Error, HttpRequest, HttpResponse,
};
use actix_web_actors::{ws, ws::WebsocketContext};
use redis_async::resp_array;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
    color: String,
}

pub struct Session {
    redis: Addr<RedisActor>,
}

impl Session {
    fn new(redis: Addr<RedisActor>) -> Self {
        Session { redis }
    }
}

#[derive(Message)]
#[rtype(result = "Result<(), ()>")]
struct SetPoint(String);

impl Actor for Session {
    type Context = WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Session {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                ctx.notify(SetPoint(text.to_string()));
                ctx.text(text);
            }
            Err(err) => println!("Protocol error: {:?}", err),
            u => println!("Unexpected message: {:?}", u),
        }
    }
}

impl Handler<SetPoint> for Session {
    type Result = Result<(), ()>;
    fn handle(&mut self, msg: SetPoint, ctx: &mut Self::Context) -> Self::Result {
        let redis = self.redis.clone();
        let fut = async move {
            let data: Point = serde_json::from_str(&msg.0).unwrap();
            redis
                .send(Command(resp_array![
                    "SET",
                    format!("{}:{}", data.x, data.y),
                    data.color
                ]))
                .await
                .unwrap()
                .unwrap();
        };
        ctx.spawn(actix::fut::wrap_future::<_, Self>(fut));
        Ok(())
    }
}

#[get("")]
pub async fn route(
    req: HttpRequest,
    stream: web::Payload,
    redis: Data<Addr<RedisActor>>,
) -> Result<HttpResponse, Error> {
    ws::start(Session::new(redis.get_ref().clone()), &req, stream)
}
