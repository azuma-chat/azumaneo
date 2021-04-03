use crate::models::error::AzumaError;
use crate::models::textchannel::TextChannel;
use actix::prelude::*;
use sqlx::{query_as, PgPool};
use std::collections::{HashMap};
use uuid::Uuid;
use crate::models::message::ChatMessage;
use actix_broker::{BrokerIssue};

#[derive(Clone, Debug)]
pub struct ChannelHandler {
    db: PgPool,
    textchannels: HashMap<Uuid, TextChannel>,
}

#[derive(Message, Clone)]
#[rtype(response = "()")]
pub struct MessageSentEvent(pub ChatMessage);

#[derive(Message, Clone)]
#[rtype(response = "()")]
pub struct MessageSendRequest(pub ChatMessage);


#[derive(Message)]
#[rtype(response = "()")]
struct UpdateSelf(ChannelHandler);

impl ChannelHandler {
    pub fn new(db: PgPool) -> ChannelHandler {
        ChannelHandler {
            db: db,
            textchannels: Default::default(),
        }
    }
}

impl Actor for ChannelHandler {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let s = self.clone();
        let mut t = self.clone();
        async move { load_textchannels(&s.db).await }
            .into_actor(self)
            .map(move |res, _act, ctx| {
                let res = match res {
                    Ok(res) => res,
                    Err(err) => {
                        panic!("Unable to load textchannels from DB: {}", err);
                    }
                };
                t.textchannels = res;
                ctx.address().do_send(UpdateSelf(t))
            })
            .spawn(ctx);
    }
}

impl Handler<UpdateSelf> for ChannelHandler {
    type Result = ();

    fn handle(&mut self, msg: UpdateSelf, _ctx: &mut Context<Self>) -> Self::Result {
        *self = msg.0;
    }
}

impl Handler<MessageSendRequest> for ChannelHandler {
    type Result = ();

    fn handle(&mut self, msg: MessageSendRequest, _ctx: &mut Context<Self>) -> Self::Result {
        self.issue_system_async(MessageSentEvent(msg.0));
    }
}


//helper functions

async fn load_textchannels(db: &PgPool) -> Result<HashMap<Uuid, TextChannel>, AzumaError> {
    let query = query_as!(TextChannel, "SELECT * FROM textchannels")
        .fetch_all(db)
        .await;

    let vec = match query {
        Ok(vec) => vec,
        Err(err) => {
            return Err(AzumaError::InternalServerError {
                source: Box::new(err),
            })
        }
    };

    let mut hs = HashMap::new();
    for channel in vec.iter() {
        hs.insert(channel.id, channel.clone());
    }
    Ok(hs)
}
