use sqlx::{query_as, PgPool};
use uuid::Uuid;

use crate::models::error::AzumaError;

//TODO: permission int is still missing
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct TextChannel {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

impl TextChannel {
    pub async fn new(db: &PgPool, name: String, desc: Option<String>) -> Result<Self, AzumaError> {
        match desc {
            None => Ok(query_as!(
                TextChannel,
                "INSERT INTO textchannels (name) VALUES ($1) RETURNING *",
                name,
            )
            .fetch_one(db)
            .await?),
            Some(desc) => Ok(query_as!(
                TextChannel,
                "INSERT INTO textchannels (name, description) VALUES ($1, $2) RETURNING *",
                name,
                desc
            )
            .fetch_one(db)
            .await?),
        }
    }

    pub async fn get_by_id(db: &PgPool, id: &Uuid) -> Result<Self, AzumaError> {
        Ok(
            query_as!(TextChannel, "SELECT * FROM textchannels WHERE id = $1", id)
                .fetch_one(db)
                .await?,
        )
    }

    pub async fn load_all(db: &PgPool) -> Result<Vec<Self>, AzumaError> {
        Ok(query_as!(TextChannel, "SELECT * FROM textchannels")
            .fetch_all(db)
            .await?)
    }
}
