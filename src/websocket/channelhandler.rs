use crate::models::error::AzumaError;
use crate::models::message::ChatMessage;
use crate::models::textchannel::TextChannel;
use actix::prelude::*;
use actix_broker::BrokerIssue;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

/// The ChannelHandler is responsible for everything related to channels (e.g. sending messages, typing events etc).
#[derive(Clone, Debug)]
pub struct ChannelHandler {
    db: PgPool,
    textchannels: HashMap<Uuid, TextChannel>,
}

/// This event is fired by the channelhandler actor if there is a message to be sent
#[derive(Message, Clone)]
#[rtype(response = "()")]
pub struct MessageSentEvent(pub ChatMessage);

/// This actor message is used by a ws actor in order to indicate that the user requested to send a message
#[derive(Message, Clone)]
#[rtype(response = "()")]
pub struct MessageSendRequest(pub ChatMessage);

/// `UpdateSelf` is used to update the channelhandler out of an asynchronous context
///
/// **NOTE:** **Everyone with access to the channelhandlers address can send this message** and currently there are **no permission checks** in place in order to check the messages origin!
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
/// We need to implement [`Actor`] for [`ChannelHandler`] otherwise it wouldn't be able to act like an actor
impl Actor for ChannelHandler {
    type Context = Context<Self>;
    /// Triggered on actor startup
    /// loads up all the channels from the database and stores them
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

/// Update self
impl Handler<UpdateSelf> for ChannelHandler {
    type Result = ();

    fn handle(&mut self, msg: UpdateSelf, _ctx: &mut Context<Self>) -> Self::Result {
        *self = msg.0;
    }
}

/// This handler processes the message sending requests from clients
impl Handler<MessageSendRequest> for ChannelHandler {
    type Result = ();

    fn handle(&mut self, msg: MessageSendRequest, _ctx: &mut Context<Self>) -> Self::Result {
        self.issue_system_async(MessageSentEvent(msg.0));
    }
}

/// Helper function which is used during startup to load up all the textchannels from the db
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
