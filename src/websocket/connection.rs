use actix::{
    Actor, ActorContext, ActorFutureExt, AsyncContext, ContextFutureSpawner, Handler,
    Message as MessageMacro, StreamHandler, WrapFuture,
};
use actix_web::web;
use actix_web_actors::ws::{self, Message};
use log::info;
use uuid::Uuid;

use crate::models::error::AzumaError;
use crate::models::message::ChatMessage;
use crate::models::session::Session;
use crate::models::stateactor::{AddUserSession, RemoveUserSession};
use crate::models::textchannel::TextChannel;
use crate::models::ws::{AwspRequestMessage, AwspResponseMessage};
use crate::websocket::broker::{MassSubChannel, UnsubAll};
use crate::AzumaState;

pub struct Ws {
    pub data: web::Data<AzumaState>,
    pub subject: Option<Uuid>,
    /// Each connection has its own id to seperate it from the other sessions a user could have
    pub connection_id: Uuid,
}

#[derive(MessageMacro)]
#[rtype(result = "()")]

struct SetSubject(Option<Uuid>);

impl Actor for Ws {
    type Context = ws::WebsocketContext<Self>;

    fn stopped(&mut self, ctx: &mut Self::Context) {
        self.data.broker.do_send(UnsubAll {
            addr: ctx.address(),
        });
        match self.subject {
            Some(uuid) => {
                self.data.state.do_send(RemoveUserSession {
                    subject: uuid,
                    connection_id: self.connection_id,
                });
            }
            None => (),
        };
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Ws {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(Message::Ping(msg)) => ctx.pong(&msg),
            Ok(Message::Close(_)) => {
                ctx.close(None);
                ctx.stop();
            }
            Ok(Message::Text(text)) => {
                let data = self.data.clone();
                let addr = ctx.address();
                let connection_id = self.connection_id;
                async move {
                    match serde_json::from_str::<AwspRequestMessage>(&text) {
                        Ok(AwspRequestMessage::Authenticate { token }) => {
                            let session = match Session::get_and_renew(&token, &data.db).await {
                                Ok(session) => session,
                                Err(AzumaError::NotFound) => return Err(AzumaError::Unauthorized),
                                Err(err) => return Err(err),
                            };

                            let channels = match TextChannel::get_all(&data.db).await {
                                Ok(vec) => vec,
                                Err(err) => return Err(err),
                            };
                            let mut topics = Vec::new();
                            for channel in channels {
                                topics.push(channel.id)
                            }
                            info!(target: "Websocket", "Authenticated websocket session of user '{}'", session.subject);
                            data.broker
                                .send(MassSubChannel {
                                    addr: addr.clone(),
                                    session: session.clone(),
                                    topics,
                                })
                                .await?;
                            data.state.do_send(AddUserSession { subject: session.subject, addr: addr.clone(), connection_id });
                            addr.do_send(SetSubject(Some(session.subject)));

                            let res = AwspResponseMessage::Welcome;
                            Ok(res)
                        }
                        Err(_) => Err(AzumaError::BadRequest),
                    }
                }
                .into_actor(self)
                .map(|result, _actor, ctx| {
                    let res = match result {
                        Ok(res) => res,
                        Err(err) => AwspResponseMessage::Error {
                            message: format!("{}", err),
                        },
                    };

                    ctx.text(
                        serde_json::to_string(&res)
                            .expect("couldn't serialize AwspResponseMessage"),
                    );
                })
                .spawn(ctx);
            }
            _ => (),
        }
    }
}

impl Handler<ChatMessage> for Ws {
    type Result = ();

    fn handle(&mut self, msg: ChatMessage, ctx: &mut Self::Context) {
        let res = AwspResponseMessage::Message(msg);
        ctx.text(serde_json::to_string(&res).expect("couldn't serialize AwspResponseMessage"));
    }
}

impl Handler<SetSubject> for Ws {
    type Result = ();
    fn handle(&mut self, msg: SetSubject, _ctx: &mut Self::Context) {
        self.subject = msg.0;
    }
}
