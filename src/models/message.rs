use actix::prelude::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Message, Clone)]
#[rtype(response = "()")]
pub struct ChatMessage {
    pub id: Uuid,
    pub author: Uuid,
    pub channel: Uuid,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}
