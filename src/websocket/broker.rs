use crate::{
    models::{message::ChatMessage, session::Session},
    websocket::connection::Ws,
};
use actix::{Actor, Addr, Context, Handler, Message};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};
use uuid::Uuid;

struct SubManager<S: Clone + Eq + Hash, T: Clone + Eq + Hash> {
    subscribers: HashMap<S, HashSet<T>>,
    topics: HashMap<T, HashSet<S>>,
}

impl<S: Clone + Eq + Hash, T: Clone + Eq + Hash> SubManager<S, T> {
    fn new() -> Self {
        SubManager {
            subscribers: HashMap::new(),
            topics: HashMap::new(),
        }
    }

    fn sub(&mut self, subscriber: &S, topic: &T) {
        if let Some(t) = self.subscribers.get_mut(&subscriber) {
            t.insert(topic.clone());
        } else {
            let mut t = HashSet::new();
            t.insert(topic.clone());
            self.subscribers.insert(subscriber.clone(), t);
        }

        if let Some(s) = self.topics.get_mut(&topic) {
            s.insert(subscriber.clone());
        } else {
            let mut s = HashSet::new();
            s.insert(subscriber.clone());
            self.topics.insert(topic.clone(), s);
        }
    }

    fn unsub(&mut self, subscriber: &S, topic: &T) {
        if let Some(t) = self.subscribers.get_mut(subscriber) {
            t.remove(topic);
        }

        if let Some(s) = self.topics.get_mut(topic) {
            s.remove(subscriber);
        }
    }

    fn unsub_all(&mut self, subscriber: &S) {
        if let Some(t) = self.subscribers.get_mut(subscriber) {
            for topic in t.iter() {
                if let Some(s) = self.topics.get_mut(topic) {
                    s.remove(subscriber);
                }
            }
            self.subscribers.remove(subscriber);
        }
    }

    fn get_subs(&self, topic: &T) -> Vec<&S> {
        if let Some(s) = self.topics.get(topic) {
            s.iter().collect()
        } else {
            Vec::new()
        }
    }
}

pub struct Broker {
    channel_subs: SubManager<Addr<Ws>, Uuid>,
}

impl Broker {
    pub fn new() -> Self {
        Broker {
            channel_subs: SubManager::new(),
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
            Broadcast::ChatMessage(x) => {
                for sub in self.channel_subs.get_subs(&x.channelid) {
                    sub.do_send(x.clone());
                }
            }
        }
    }
}
