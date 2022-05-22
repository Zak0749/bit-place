use std::collections::HashMap;

use super::session::{SendPoint, Session};
use actix::{Actor, Addr, AsyncContext, Context, Handler, Message};
use actix_redis::{resp_array, Command, RedisActor};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct Server {
    sessions: HashMap<Uuid, Addr<Session>>,
    redis: Addr<RedisActor>,
}

impl Server {
    pub fn new(redis: Addr<RedisActor>) -> Addr<Self> {
        Server {
            sessions: HashMap::new(),
            redis,
        }
        .start()
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "Result<(), ()>")]
pub struct Connect {
    pub id: Uuid,
    pub session: Addr<Session>,
}

impl Handler<Connect> for Server {
    type Result = Result<(), ()>;
    fn handle(&mut self, msg: Connect, _ctx: &mut Context<Self>) -> Self::Result {
        self.sessions.insert(msg.id, msg.session);
        Ok(())
    }
}

#[derive(Message)]
#[rtype(result = "Result<(), ()>")]
pub struct Disconnect {
    pub id: Uuid,
}

impl Handler<Disconnect> for Server {
    type Result = Result<(), ()>;
    fn handle(&mut self, msg: Disconnect, _ctx: &mut Context<Self>) -> Self::Result {
        self.sessions.remove(&msg.id);

        Ok(())
    }
}

#[derive(Message)]
#[rtype(result = "Result<(), ()>")]
pub struct SetPoint {
    pub point: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
    color: String,
}

impl Handler<SetPoint> for Server {
    type Result = Result<(), ()>;

    fn handle(&mut self, msg: SetPoint, ctx: &mut Context<Self>) -> Self::Result {
        self.sessions.iter().for_each(|(_, session)| {
            session.do_send(SendPoint {
                point: msg.point.clone(),
            });
        });
        let redis = self.redis.clone();
        let fut = async move {
            let data: Point = serde_json::from_str(&msg.point).unwrap();
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
