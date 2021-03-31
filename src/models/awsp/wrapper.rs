use std::collections::HashMap;

use actix::Message;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Message, Clone)]
#[rtype(result = "()")]
pub struct AwspWrapper {
    pub version: String,
    pub msg_type: AwspMsgType,
    pub content: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AwspMsgType {
    ///Used to authenticate the websocket session before any other communication is allowed
    Auth,

    ///Indicates that a `Auth` request was successful
    Welcome,

    ///This is used when a client is attempting to send a message
    SendMessage,

    ///This is used when a client send a message and the server is now broadcasting to all users with access to the specific channel
    UserMessage,

    ///This is used if a client attempts to change it's online status
    ChangeOnlineStatus,

    ///This is the message the server sends to the clients when a user changed its online
    UserOnlineStatus,

    ///This is fired whenever a user starts or stops to type
    SendTyping,

    ///This is fired when a user has stopped typing
    StopTyping,

    ///Indicates that a error has occured. For more details look at the `errortype` field of the content
    Error,
}
