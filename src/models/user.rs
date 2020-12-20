use crate::models::rejection::AzumaRejection;
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
    pub async fn new(name: &str, password: &str, db: &PgPool) -> Result<User, AzumaRejection> {
        // TODO: create new user
        let hashed_password = argon2::hash_encoded(
            password.as_bytes(),
            &random::<[u8; 8]>(),
            &argon2::Config::default(),
        )
        .unwrap();

        let user = query_as!(
            User,
            "INSERT INTO users (name, password) values ($1, $2) ON CONFLICT DO NOTHING RETURNING *",
            name,
            hashed_password
        )
        .fetch_optional(db)
        .await
        .unwrap()
        .unwrap();

        Ok(user)
    }

    pub async fn get(name: String) -> Result<(), AzumaRejection> {
        // TODO: get user by name
        Ok(())
    }

    pub async fn get_by_id(id: String) -> Result<(), AzumaRejection> {
        // TODO: get user by id
        Ok(())
    }

    /*pub async fn update(id: u64, updates: UpdatableUser) -> Result<(), AzumaRejection> {
        // TODO: update user
        Ok(())
    }*/
}
