use actix::{Actor, Addr, Handler, StreamHandler};
use actix_web::{web::{self, Data}, HttpRequest, HttpResponse};
use actix_web_actors::ws::{self, WebsocketContext, WsResponseBuilder};
use serde::Serialize;
use std::fmt::Debug;
use actix::Message;
use std::sync::{Arc, Mutex};

pub struct MessageWs {}

impl Actor for MessageWs {
    type Context = WebsocketContext<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Payload<T> {
    pub payload: T,
}

impl<T> Handler<Payload<T>> for MessageWs
where
    T: Serialize + Debug,
{
    type Result = ();

    fn handle(&mut self, msg: Payload<T>, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(serde_json::to_string(&msg.payload).expect("Unable to serialize"));
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MessageWs {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match item {
            _ => ctx.text("Messages are not accepted"),
        }
    }
}

pub async fn ws_message(
    req: HttpRequest,
    stream: web::Payload,
    ws_state: Data<Arc<Mutex<Vec<Addr<MessageWs>>>>>,
) -> HttpResponse {
    let (addr, res) = WsResponseBuilder::start_with_addr(WsResponseBuilder::new(MessageWs {}, &req, stream)).unwrap();
    ws_state.lock().unwrap().push(addr);
    res
}
