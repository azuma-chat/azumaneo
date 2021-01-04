use crate::models::{error::AzumaError, session::Session, user::User};
use crate::AzumaState;
use actix_web::{web, HttpResponse};
use argon2::verify_encoded;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RegisterUserRequest {
    name: String,
    password: String,
}

#[derive(Serialize)]
pub struct RegisterUserResponse {
    token: Uuid,
}

pub async fn register_user(
    data: web::Data<AzumaState>,
    request: web::Json<RegisterUserRequest>,
) -> Result<HttpResponse, AzumaError> {
    let user = User::new(&request.name, &request.password, &data.db).await?;
    let session = Session::new(&user, &data.db).await?;

    let response_body = RegisterUserResponse {
        token: session.token,
    };
    Ok(HttpResponse::Created().json(response_body))
}

#[derive(Deserialize)]
pub struct LoginUserRequest {
    name: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginUserResponse {
    token: Uuid,
}

pub async fn login_user(
    data: web::Data<AzumaState>,
    request: web::Json<LoginUserRequest>,
) -> Result<HttpResponse, AzumaError> {
    let user = User::get_by_name(&request.name, &data.db).await?;

    if verify_encoded(&user.password, request.password.as_bytes())? {
        let session = Session::new(&user, &data.db).await?;

        let response_body = LoginUserResponse {
            token: session.token,
        };
        Ok(HttpResponse::Ok().json(response_body))
    } else {
        Err(AzumaError::Forbidden)
    }
}

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
