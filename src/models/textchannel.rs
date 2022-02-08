use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{query, query_as, PgPool};
use uuid::Uuid;

use crate::models::error::AzumaError;

// TODO: permission int is still missing
#[derive(Clone, Debug, Serialize)]
pub struct TextChannel {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl TextChannel {
    pub async fn new(
        db: &PgPool,
        name: &str,
        description: Option<&str>,
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

    pub async fn remove(db: &PgPool, id: &Uuid) -> Result<(), AzumaError> {
        query!("DELETE FROM textchannels WHERE id = $1", id).execute(db).await?;
        Ok(())
    }
}
