use crate::models::error::AzumaError;
use crate::AzumaState;
use actix::prelude::*;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{query_as, PgPool};
use uuid::Uuid;

/// This represents a chat message a user sends to a given channel
#[derive(Clone, Message, Serialize)]
#[rtype(response = "()")]
pub struct ChatMessage {
    pub id: Uuid,
    pub authorid: Uuid,
    pub channelid: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl ChatMessage {
    /// Send a new message into a textchannel
    pub async fn new(self, state: &AzumaState) -> Result<(), AzumaError> {
        query_as!(
            ChatMessage,
            "INSERT INTO messages (authorid, channelid, content) VALUES ($1, $2, $3)",
            self.authorid,
            self.channelid,
            self.content
        )
        .execute(&state.db)
        .await?;
        Ok(())
    }

    pub async fn get_msgs(
        after: Option<&Uuid>,
        limit: Option<i32>,
        channelid: &Uuid,
        db: &PgPool,
    ) -> Result<Vec<ChatMessage>, AzumaError> {
        match after {
            Some(after) => {
                let messages: Vec<ChatMessage> = query_as!(ChatMessage, "SELECT * FROM messages WHERE created_at < (SELECT created_at from messages WHERE id = $1) AND channelid = $2  ORDER BY created_at ASC LIMIT LEAST(100, COALESCE($3, 50))",
                after,
                channelid,
                limit)
                    .fetch_all(db)
                    .await?;
                Ok(messages)
            }
            None => {
                let messages: Vec<ChatMessage> = query_as!(
                    ChatMessage,
                    "SELECT * FROM messages WHERE channelid = $1 ORDER BY created_at ASC LIMIT LEAST(100, COALESCE($2, 50))",
                    channelid,
                    limit
                )
                .fetch_all(db)
                .await?;
                Ok(messages)
            }
        }
    }
}
