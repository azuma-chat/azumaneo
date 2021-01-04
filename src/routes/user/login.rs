use crate::{
    models::{error::AzumaError, session::Session, user::User},
    AzumaState,
};
use actix_web::{web, HttpResponse};
use argon2::verify_encoded;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
