use chrono::{DateTime, Utc};
use sqlx::{query_as, PgPool};
use uuid::Uuid;

use crate::models::error::AzumaError;

// TODO: permission int is still missing
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct TextChannel {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl TextChannel {
    pub async fn new(
        db: &PgPool,
        name: String,
        description: Option<String>,
    ) -> Result<Self, AzumaError> {
        let text_channel = query_as!(
            TextChannel,
            "INSERT INTO textchannels (name, description) VALUES ($1, $2) RETURNING *",
            name,
            description
        )
        .fetch_one(db)
        .await?;

        Ok(text_channel)
    }

    pub async fn get_by_id(db: &PgPool, id: &Uuid) -> Result<Self, AzumaError> {
        // TODO: We need to implement some sort of cache later on
        let text_channel = query_as!(TextChannel, "SELECT * FROM textchannels WHERE id = $1", id)
            .fetch_optional(db)
            .await?;

        text_channel.ok_or(AzumaError::NotFound)
    }

    pub async fn get_all(db: &PgPool) -> Result<Vec<Self>, AzumaError> {
        let text_channels = query_as!(TextChannel, "SELECT * FROM textchannels")
            .fetch_all(db)
            .await?;

        Ok(text_channels)
    }
}
