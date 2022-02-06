use actix::{Actor, Addr, Context, Handler, Message};
use uuid::Uuid;

use crate::models::message::ChatMessage;
use crate::models::pub_sub::PubSub;
use crate::models::session::Session;
use crate::websocket::connection::Ws;

pub struct Broker {
    channel_subs: PubSub<Addr<Ws>, Uuid>,
}

impl Broker {
    pub fn new() -> Self {
        Broker {
            channel_subs: PubSub::new(),
        }
    }
}

impl Actor for Broker {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "")]
/// Used to sub to multiple textchannels at once.<br>
/// **Note:** This is a temporary solution and not intended to be permanent
pub struct MassSubChannel {
    pub addr: Addr<Ws>,
    pub session: Session,
    pub topics: Vec<Uuid>,
}

impl Handler<MassSubChannel> for Broker {
    type Result = ();

    fn handle(&mut self, msg: MassSubChannel, _ctx: &mut Self::Context) {
        // Null UUID is used as a default channel
        self.channel_subs.sub(&msg.addr, &Uuid::from_bytes([0; 16]));
        for channel in msg.topics {
            self.channel_subs.sub(&msg.addr, &channel)
        }
    }
}

#[derive(Message)]
#[rtype(result = "")]
pub struct UnsubAll {
    pub addr: Addr<Ws>,
}

impl Handler<UnsubAll> for Broker {
    type Result = ();

    fn handle(&mut self, msg: UnsubAll, _ctx: &mut Self::Context) {
        self.channel_subs.unsub_all(&msg.addr);
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub enum Broadcast {
    ChatMessage(ChatMessage),
}

impl Handler<Broadcast> for Broker {
    type Result = ();

    fn handle(&mut self, msg: Broadcast, _ctx: &mut Self::Context) {
        match msg {
            Broadcast::ChatMessage(m) => {
                for sub in self.channel_subs.get_subs(&m.channel) {
                    sub.do_send(m.clone());
                }
            }
        }
    }
}
