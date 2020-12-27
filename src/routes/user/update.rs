use crate::{
    models::{error::AzumaError, user::User},
    AzumaState,
};
use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    // TODO: remove id when sessions are implemented
    id: Uuid,
    name: Option<String>,
    password: Option<String>,
}

#[derive(Serialize)]
pub struct UpdateUserResponse {
    id: Uuid,
    name: String,
    created: DateTime<Utc>,
}

pub async fn update_user(
    data: web::Data<AzumaState>,
    request: web::Json<UpdateUserRequest>,
) -> Result<HttpResponse, AzumaError> {
    let user = User::get_by_id(&request.id, &data.db)
        .await?
        .update(
            request.name.as_deref(),
            request.password.as_deref(),
            &data.db,
        )
        .await?;

    let response_body = UpdateUserResponse {
        id: user.id,
        name: user.name,
        created: user.created,
    };
    Ok(HttpResponse::Ok().json(response_body))
}
