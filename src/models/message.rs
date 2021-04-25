use actix::prelude::*;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

/// This represents a chat message a user sends to a given channel
#[derive(Clone, Message, Serialize)]
#[rtype(response = "()")]
pub struct ChatMessage {
    pub id: Uuid,
    pub author: Uuid,
    pub channel: Uuid,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}
