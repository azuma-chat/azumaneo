use actix::{
    fut, Actor, ActorContext, ActorFuture, AsyncContext, ContextFutureSpawner, Handler, Running,
    StreamHandler, WrapFuture,
};
use actix_web::web;
use actix_web_actors::ws;
use serde::Deserialize;
use uuid::Uuid;

use crate::models::awsp::error::AwspErrorType;
use crate::models::awsp::wrapper::AwspMsgType::Auth;
use crate::models::awsp::wrapper::{AwspMsgType, AwspWrapper};
use crate::models::error::AzumaError;
use crate::models::session::Session;
use crate::websocket::chatserver;
use crate::AzumaState;
use std::ops::Deref;
use std::str::FromStr;
use crate::websocket::chatserver::Message;
use std::collections::HashMap;
use tokio::sync::oneshot;
use std::thread::{Thread, sleep};
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct Ws {
    pub session_id: Uuid,
    pub user_id: Option<Uuid>,
    pub state: web::Data<AzumaState>,
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
        self.state
            .srv
            .send(chatserver::Connect {
                addr: addr.recipient(),
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
                println!("{}", text);
                let wrapper: AwspWrapper = match serde_json::from_str(text.trim()) {
                    Ok(wrapper) => wrapper,
                    //TODO: don't make worker crash on unparsable input
                    Err(_err) => {
                        self.state.srv.do_send(AwspWrapper {
                            version: self.state.constants.awsp_version.to_string(),
                            msg_type: AwspMsgType::Error,
                            content: AwspErrorType::BadRequest.into_hm(),
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
                                content: AwspErrorType::Unauthorized.into_hm(),
                            });
                            return;
                        } else {

                            //TODO impl auth mechanism
                            

                        }
                    }
                    Some(usrid) => {
                        println!("userid: {:?}", usrid)
                    }
                };
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
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
