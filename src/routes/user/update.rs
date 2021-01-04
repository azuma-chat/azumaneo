use crate::{
    models::{error::AzumaError, session::Session, user::User},
    AzumaState,
};
use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    name: Option<String>,
    password: Option<String>,
}

#[derive(Serialize)]
pub struct UpdateUserResponse {
    id: Uuid,
    name: String,
    created_at: DateTime<Utc>,
}

pub async fn update_user(
    data: web::Data<AzumaState>,
    request: web::Json<UpdateUserRequest>,
    session: Session,
) -> Result<HttpResponse, AzumaError> {
    let mut user = User::get_by_id(&session.subject, &data.db).await?;
    user.update(
        request.name.as_deref(),
        request.password.as_deref(),
        &data.db,
    )
    .await?;

    let response_body = UpdateUserResponse {
        id: user.id,
        name: user.name,
        created_at: user.created_at,
    };
    Ok(HttpResponse::Ok().json(response_body))
}
