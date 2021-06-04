use crate::models::error::{Argon2idError, AzumaError};
use chrono::{DateTime, Utc};
use log::info;
use serde::Serialize;
use sodiumoxide::crypto::pwhash::argon2id13;
use sqlx::{query_as, types::Uuid, FromRow, PgPool};

/// The representation of a user account
#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    #[serde(skip)]
    pub password: Vec<u8>,
    pub created_at: DateTime<Utc>,
    // TODO: add icon and status properties
}

impl User {
    /// Create a new user in the database and return it
    pub async fn new(name: &str, password: &str, db: &PgPool) -> Result<Self, AzumaError> {
        let hashed_password = argon2id13::pwhash(
            password.as_bytes(),
            argon2id13::OPSLIMIT_INTERACTIVE,
            argon2id13::MEMLIMIT_INTERACTIVE,
        )
        .map_err(|_| Argon2idError)?;

        let user = query_as!(
            User,
            "INSERT INTO users (name, password) values ($1, $2) RETURNING *",
            name,
            hashed_password.as_ref()
        )
        .fetch_one(db)
        .await?;
        info!(target: "Access Control", "Created user with name '{}' and id {}", name, user.id);

        Ok(user)
    }

    /// Get a user by his/her id
    pub async fn get_by_id(id: &Uuid, db: &PgPool) -> Result<Self, AzumaError> {
        let user = query_as!(User, "SELECT * FROM users WHERE id = $1", id)
            .fetch_optional(db)
            .await?;

        user.ok_or(AzumaError::NotFound)
    }

    /// Get a user by his/her name
    pub async fn get_by_name(name: &str, db: &PgPool) -> Result<User, AzumaError> {
        let user = query_as!(User, "SELECT * FROM users WHERE name = $1", name)
            .fetch_optional(db)
            .await?;

        user.ok_or(AzumaError::NotFound)
    }

    /// Update a user
    pub async fn update(
        &mut self,
        name: Option<&str>,
        password: Option<&str>,
        db: &PgPool,
    ) -> Result<(), AzumaError> {
        let hashed_password = match password {
            Some(password) => {
                let hashed_password = argon2id13::pwhash(
                    password.as_bytes(),
                    argon2id13::OPSLIMIT_INTERACTIVE,
                    argon2id13::MEMLIMIT_INTERACTIVE,
                )
                .map_err(|_| Argon2idError)?;
                Some(hashed_password)
            }
            None => None,
        };

        let user = query_as!(
            User,
            "UPDATE users SET name = COALESCE($1, name), password = COALESCE($2, password) WHERE id = $3 RETURNING *",
            name,
            hashed_password.as_ref().map(|hp| hp.as_ref()),
            self.id
        )
            .fetch_one(db)
            .await?;
        info!(target: "Access Control", "Updated user '{}'", user.id);
        *self = user;
        Ok(())
    }
}
