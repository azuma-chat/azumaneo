use crate::models::message::ChatMessage;
use actix::Message;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum AwspRequestMessage {
    Authenticate { token: Uuid },
}

#[derive(Serialize)]
#[serde(tag = "type", content = "content")]
pub enum AwspResponseMessage {
    Error { message: String },
    Message(ChatMessage),
    Welcome,
}

#[derive(Serialize, Deserialize, Debug, Message, Clone)]
#[rtype(result = "()")]
pub struct AwspWrapper {
    pub version: String,
    pub msg_type: AwspMessageTypex,
    pub content: HashMap<String, String>,
}

impl ToString for AwspWrapper {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("An error occurred while translating message to json")
    }
}

/// The `AwspMessageTypex` tells the client what the server tries to communicate and what fields to expect in the `content` field of the wrapper
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AwspMessageTypex {
    ///Used to authenticate the websocket session before any other communication is allowed
    Auth,

    ///Indicates that a `Auth` request was successful
    Welcome,

    ///This is used when a client is attempting to send a message
    SendMessage,

    ///This variant is used to indicate that a message was sent
    MessageSent,

    /// If a users onlinestatus is changed, this is sent to all clients
    ChangeOnlineStatus,

    ///This is fired whenever a user starts or stops to type
    SendTyping,

    ///This is fired when a user has stopped typing
    StopTyping,

    ///Indicates that a error has occured. For more details look at the `errortype` field of the content
    Error,
}
