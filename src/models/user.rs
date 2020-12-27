use crate::models::error::AzumaError;
use chrono::{DateTime, Utc};
use rand::random;
use sqlx::{query_as, types::Uuid, FromRow, PgPool};

#[derive(Debug, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub password: String,
    pub created: DateTime<Utc>,
    // TODO: add icon and status properties
}

impl User {
    pub async fn new(name: &str, password: &str, db: &PgPool) -> Result<User, AzumaError> {
        let hashed_password = argon2::hash_encoded(
            password.as_bytes(),
            &random::<[u8; 8]>(),
            &argon2::Config::default(),
        )?;

        let user = query_as!(
            User,
            "INSERT INTO users (name, password) values ($1, $2) RETURNING *",
            name,
            hashed_password
        )
        .fetch_one(db)
        .await?;

        Ok(user)
    }

    pub async fn get_by_id(id: &Uuid, db: &PgPool) -> Result<User, AzumaError> {
        let user = query_as!(User, "SELECT * FROM users WHERE id = $1", id)
            .fetch_optional(db)
            .await?;

        user.ok_or(AzumaError::NotFound)
    }

    pub async fn get(name: &str, db: &PgPool) -> Result<User, AzumaError> {
        let user = query_as!(User, "SELECT * FROM users WHERE name = $1", name)
            .fetch_optional(db)
            .await?;

        user.ok_or(AzumaError::NotFound)
    }

    pub async fn update(
        &self,
        name: Option<&str>,
        password: Option<&str>,
        db: &PgPool,
    ) -> Result<User, AzumaError> {
        let mut hashed_password = None;
        if let Some(password) = password {
            hashed_password = Some(argon2::hash_encoded(
                password.as_bytes(),
                &random::<[u8; 8]>(),
                &argon2::Config::default(),
            )?);
        }

        let user = query_as!(
            User,
            "UPDATE users SET name = COALESCE($1, name), password = COALESCE($2, password) WHERE id = $3 RETURNING *",
            name,
            hashed_password,
            self.id
        )
        .fetch_one(db)
        .await?;
        Ok(user)
    }
}
