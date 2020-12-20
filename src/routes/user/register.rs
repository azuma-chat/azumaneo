use crate::models::{error::AzumaError, user::User};
use crate::AzumaState;
use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterUserRequest {
    name: String,
    password: String,
}

pub async fn register_user(
    data: web::Data<AzumaState>,
    request: web::Json<RegisterUserRequest>,
) -> Result<HttpResponse, AzumaError> {
    let _ = User::new(&request.name, &request.password, &data.db).await?;

    // TODO: return session for newly created user
    Ok(HttpResponse::Ok().body(""))
}
