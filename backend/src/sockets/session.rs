use crate::sockets::server::{self, Server};
use actix::{
    clock::Instant, Actor, ActorContext, Addr, AsyncContext, Handler, Message, Running,
    StreamHandler,
};
use actix_web_actors::{ws, ws::WebsocketContext};
use std::time::Duration;
use uuid::Uuid;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct Session {
    pub id: Uuid,
    hb: Instant,
    server: Addr<Server>,
}

impl Session {
    pub fn new(server: Addr<Server>) -> Self {
        Session {
            hb: Instant::now(),
            server,
            id: Uuid::new_v4(),
        }
    }

    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        let id = self.id;
        ctx.run_interval(HEARTBEAT_INTERVAL, move |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                act.server.do_send(server::Disconnect { id });
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for Session {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.server.do_send(server::Connect {
            session: ctx.address(),
            id: self.id,
        });
        self.hb(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.server.do_send(server::Disconnect { id: self.id });
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Session {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                self.server.do_send(server::SetPoint {
                    point: text.to_string(),
                });
            }
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Close(_)) => {
                ctx.stop();
            }
            _ => (),
        }
    }
}

#[derive(Message)]
#[rtype(result = "Result<(), ()>")]
pub struct SendPoint {
    pub point: String,
}

impl Handler<SendPoint> for Session {
    type Result = Result<(), ()>;
    fn handle(&mut self, msg: SendPoint, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.point);
        Ok(())
    }
}
