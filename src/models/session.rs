use std::{future::Future, pin::Pin};

use actix_web::{dev::Payload, web::Data, FromRequest, HttpRequest};
use chrono::{DateTime, Utc};
use sqlx::{query_as, FromRow, PgPool};
use uuid::Uuid;

use crate::{
    models::{error::AzumaError, user::User},
    AzumaState,
};

#[derive(FromRow, Clone)]
pub struct Session {
    pub token: Uuid,
    pub subject: Uuid,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl Session {
    pub async fn new(subject: &User, db: &PgPool) -> Result<Self, AzumaError> {
        let session = query_as!(
            Session,
            "INSERT INTO sessions (subject) values ($1) RETURNING *",
            subject.id
        )
        .fetch_one(db)
        .await?;

        Ok(session)
    }

    pub async fn get_and_renew(token: &Uuid, db: &PgPool) -> Result<Self, AzumaError> {
        let session = query_as!(
            Session,
            "UPDATE sessions SET expires_at = current_timestamp + (14 * interval '1 day') WHERE token = $1 AND expires_at > current_timestamp RETURNING *",
            token
        )
        .fetch_optional(db)
        .await?;

        session.ok_or(AzumaError::NotFound)
    }
}

impl FromRequest for Session {
    type Error = AzumaError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let token = req
                .headers()
                .get("Authorization")
                .ok_or(AzumaError::Unauthorized)?
                .to_str()
                .or(Err(AzumaError::Unauthorized))?
                .strip_prefix("Bearer ")
                .ok_or(AzumaError::Unauthorized)?;
            let token = Uuid::parse_str(token).or(Err(AzumaError::Unauthorized))?;

            let data = req
                .app_data::<Data<AzumaState>>()
                .expect("app data missing")
                .as_ref();

            match Session::get_and_renew(&token, &data.db).await {
                Ok(session) => Ok(session),
                Err(AzumaError::NotFound) => Err(AzumaError::Unauthorized),
                Err(err) => Err(err),
            }
        })
    }
}
