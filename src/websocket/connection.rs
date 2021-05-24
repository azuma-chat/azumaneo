use crate::{
    models::{
        awsp::wrapper::{AwspRequestMessage, AwspResponseMessage},
        error::AzumaError,
        message::ChatMessage,
        session::Session,
    },
    websocket::broker::{Connect, Disconnect},
    AzumaState,
};
use actix::{
    Actor, ActorFuture, AsyncContext, ContextFutureSpawner, Handler, StreamHandler, WrapFuture,
};
use actix_web::web;
use actix_web_actors::ws::{self, Message};

pub struct Ws {
    pub data: web::Data<AzumaState>,
}

impl Actor for Ws {
    type Context = ws::WebsocketContext<Self>;

    fn stopped(&mut self, ctx: &mut Self::Context) {
        self.data.broker.do_send(Disconnect {
            addr: ctx.address(),
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Ws {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(Message::Ping(msg)) => ctx.pong(&msg),
            Ok(Message::Close(_)) => ctx.close(None),
            Ok(Message::Text(text)) => {
                let data = self.data.clone();
                let addr = ctx.address();
                async move {
                    match serde_json::from_str::<AwspRequestMessage>(&text) {
                        Ok(AwspRequestMessage::Authenticate { token }) => {
                            let session = match Session::get_and_renew(&token, &data.db).await {
                                Ok(session) => session,
                                Err(AzumaError::NotFound) => return Err(AzumaError::Unauthorized),
                                Err(err) => return Err(err),
                            };

                            data.broker.send(Connect { addr, session }).await?;
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
        println!("connection handler");
        let res = AwspResponseMessage::Message(msg);
        ctx.text(serde_json::to_string(&res).expect("couldn't serialize AwspResponseMessage"));
    }
}
