use crate::models::message::ChatMessage;
use serde::{Deserialize, Serialize};
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
