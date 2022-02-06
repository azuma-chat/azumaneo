use actix::prelude::*;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{query_as, PgPool};
use uuid::Uuid;

use crate::models::error::AzumaError;
use crate::websocket::broker::{Broadcast, Broker};

/// This represents a chat message a user sends to a given channel
#[derive(Clone, Message, Serialize)]
#[rtype(response = "()")]
pub struct ChatMessage {
    pub id: Uuid,
    pub author: Uuid,
    pub channel: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl ChatMessage {
    pub async fn new(
        author: &Uuid,
        channel: &Uuid,
        content: &str,
        broker: &Addr<Broker>,
        db: &PgPool,
    ) -> Result<Self, AzumaError> {
        let chat_message = query_as!(
            ChatMessage,
            "INSERT INTO messages (author, channel, content) VALUES ($1, $2, $3) RETURNING *",
            author,
            channel,
            content
        )
        .fetch_one(db)
        .await?;

        broker.do_send(Broadcast::ChatMessage(chat_message.clone()));
        Ok(chat_message)
    }

    pub async fn get_messages(
        before: Option<&Uuid>,
        limit: Option<i32>,
        channel: &Uuid,
        db: &PgPool,
    ) -> Result<Vec<ChatMessage>, AzumaError> {
        let chat_messages: Vec<ChatMessage> = query_as!(ChatMessage, "SELECT * FROM messages WHERE created_at < COALESCE((SELECT created_at from messages WHERE id = $1), current_timestamp) AND channel = $2 ORDER BY created_at ASC LIMIT LEAST(100, COALESCE($3, 50))", before, channel, limit)
            .fetch_all(db)
            .await?;

        Ok(chat_messages)
    }
}
