use crate::models::{error::AzumaError, session::Session, user::User};
use crate::AzumaState;
use actix_web::{web, HttpResponse};
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
