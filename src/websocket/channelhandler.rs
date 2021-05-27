use std::collections::HashMap;

use actix::{
    Actor, ActorFuture, AsyncContext, Context, ContextFutureSpawner, Handler, Message, WrapFuture,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::textchannel::TextChannel;

pub struct ChannelHandler {
    db: PgPool,
    textchannels: HashMap<Uuid, TextChannel>,
}

#[derive(Message)]
#[rtype(response = "()")]
struct Startup {
    textchannels: Vec<TextChannel>,
}

#[doc(hidden)]
#[derive(Message)]
#[rtype(response = "()")]
pub struct Debug;

impl ChannelHandler {
    pub fn new(db: PgPool) -> Self {
        Self {
            db: db,
            textchannels: Default::default(),
        }
    }
}

impl Actor for ChannelHandler {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let db = self.db.clone();
        async move {
            let txchannels = match TextChannel::get_all(&db).await {
                Ok(channels) => channels,
                Err(err) => panic!("An error occured while loading up textchannels: {}", err),
            };
            Startup {
                textchannels: txchannels,
            }
        }
        .into_actor(self)
        .map(|result, _actor, ctx| {
            ctx.address().do_send(result);
        })
        .spawn(ctx);
    }
}

#[doc(hidden)]
impl Handler<Debug> for ChannelHandler {
    type Result = ();

    fn handle(&mut self, _msg: Debug, _ctx: &mut Self::Context) -> Self::Result {
        dbg!(&self.textchannels);
    }
}

/// This message is sent on startup which needs an asynchronous context we cannot provide without this
impl Handler<Startup> for ChannelHandler {
    type Result = ();

    fn handle(&mut self, msg: Startup, _ctx: &mut Self::Context) -> Self::Result {
        let mut channelmap = HashMap::new();
        for channel in msg.textchannels {
            channelmap.insert(channel.id, channel);
        }

        self.textchannels = channelmap;
    }
}
