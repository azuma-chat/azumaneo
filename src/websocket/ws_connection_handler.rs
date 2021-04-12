use actix::{
    fut, Actor, ActorContext, ActorFuture, AsyncContext, ContextFutureSpawner, Handler, Message,
    Running, StreamHandler, WrapFuture,
};
use actix_web::web;
use actix_web_actors::ws;
use uuid::Uuid;

use crate::models::awsp::wrapper::{AwspMsgType, AwspWrapper};
use crate::models::error::AzumaError;
use crate::models::message::ChatMessage;
use crate::models::session::Session;
use crate::websocket::channelhandler::{MessageSendRequest, MessageSentEvent};
use crate::websocket::chatserver;
use crate::AzumaState;
use actix_broker::BrokerSubscribe;
use chrono::Utc;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone, Debug, Message)]
#[rtype(response = "Ws")]
pub struct Ws {
    pub session_id: Uuid,
    pub user_id: Option<Uuid>,
    pub state: web::Data<AzumaState>,
}
#[derive(Message, Debug)]
#[rtype(response = "()")]
pub struct UpdateRequest {
    pub ws: Ws,
    pub to_update: Uuid,
}

impl Actor for Ws {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start.
    /// We register ws session with chatserver
    fn started(&mut self, ctx: &mut Self::Context) {
        // we'll start heartbeat process on session start.

        // register self in chat chatserver. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of WsChatSessionState, state is shared
        // across all routes within application
        let addr = ctx.address();
        self.subscribe_system_async::<MessageSentEvent>(ctx);
        self.state
            .srv
            .send(chatserver::Connect {
                addr,
                id: self.session_id,
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.session_id = res,
                    // something is wrong with chatserver
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify chat chatserver
        self.state.srv.do_send(chatserver::Disconnect {
            id: self.session_id,
        });
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Ws {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                let wrapper: AwspWrapper = match serde_json::from_str(text.trim()) {
                    Ok(wrapper) => wrapper,
                    Err(_err) => {
                        self.state.srv.do_send(AwspWrapper {
                            version: self.state.constants.awsp_version.to_string(),
                            msg_type: AwspMsgType::Error,
                            content: AzumaError::BadRequest.into_hm(),
                        });
                        return;
                    }
                };
                match self.user_id {
                    None => {
                        if wrapper.msg_type != AwspMsgType::Auth {
                            self.state.srv.do_send(AwspWrapper {
                                version: self.state.constants.awsp_version.to_string(),
                                msg_type: AwspMsgType::Error,
                                content: AzumaError::Unauthorized.into_hm(),
                            });
                            return;
                        } else {
                            let db = self.state.get_ref().db.clone();
                            let mut s = self.clone();
                            async move {
                                match Session::get_and_renew(
                                    &match Uuid::from_str(match wrapper.content.get("token") {
                                        None => {
                                            return Err(AzumaError::Unauthorized);
                                        }
                                        Some(token) => token,
                                    }) {
                                        Err(_) => {
                                            return Err(AzumaError::Unauthorized);
                                        }
                                        Ok(token) => token,
                                    },
                                    &db,
                                )
                                .await
                                {
                                    Ok(session) => return Ok(session.subject),
                                    Err(err) => {
                                        return Err(err);
                                    }
                                };
                            }
                            .into_actor(self)
                            .map(move |res, _act, ctx| {
                                let uuid = match res {
                                    Ok(uuid) => uuid,
                                    Err(_) => {
                                        ctx.text("error!".to_string());
                                        return;
                                    }
                                };
                                s.user_id = Some(uuid);
                                s.state.get_ref().srv.do_send(UpdateRequest {
                                    ws: s.clone(),
                                    to_update: s.session_id,
                                });
                                s.state.srv.do_send(AwspWrapper {
                                    version: s.state.constants.awsp_version.to_string(),
                                    msg_type: AwspMsgType::Welcome,
                                    content: {
                                        let mut hm: HashMap<String, String> = HashMap::new();
                                        hm.insert("userid".to_string(), format!("{:?}", s.user_id));
                                        hm
                                    },
                                });
                            })
                            .spawn(ctx);
                        }
                    }
                    Some(usrid) => {
                        self.state
                            .channelhandler
                            .do_send(MessageSendRequest(ChatMessage {
                                id: Uuid::new_v4(),
                                author: usrid,
                                channel: Default::default(),
                                content: wrapper.content.get("msg").unwrap().clone(),
                                timestamp: Utc::now(),
                            }))
                    }
                };
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                println!("closereason: {:?}", reason);
                ctx.close(reason)
            }
            _ => (),
        }
    }
}

/// Handle messages from chat chatserver, we simply send it to peer websocket
impl Handler<chatserver::Message> for Ws {
    type Result = ();

    fn handle(&mut self, msg: chatserver::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

/// Update self as requested by the chatserver actor.
/// This happens for example when authenticating after establishing a websocket connection.
impl Handler<UpdateRequest> for Ws {
    type Result = ();

    fn handle<'a>(&'a mut self, msg: UpdateRequest, _ctx: &mut Self::Context) {
        *self = msg.ws;
    }
}

impl Handler<MessageSentEvent> for Ws {
    type Result = ();

    fn handle(&mut self, msg: MessageSentEvent, ctx: &mut Self::Context) -> Self::Result {
        let mut content: HashMap<String, String> = HashMap::new();
        content.insert("author".to_string(), msg.0.author.to_string());
        content.insert("content".to_string(), msg.0.content);
        content.insert("id".to_string(), msg.0.id.to_string());
        content.insert("channel".to_string(), msg.0.channel.to_string());
        content.insert("timestamp".to_string(), msg.0.timestamp.to_string());
        let wrapper = AwspWrapper {
            version: self.state.constants.awsp_version.to_string(),
            msg_type: AwspMsgType::MessageSent,
            content,
        };
        ctx.text(wrapper.to_string());
    }
}
