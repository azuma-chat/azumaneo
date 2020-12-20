use crate::models::user::User;
use crate::ApiState;
use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterUserRequest {
    name: String,
    password: String,
}

pub async fn register_user(
    data: web::Data<ApiState>,
    request: web::Json<RegisterUserRequest>,
) -> HttpResponse {
    let _ = User::new(&request.name, &request.password, &data.db)
        .await
        .unwrap();

    // TODO: return session for newly created user
    HttpResponse::Ok().body("")
}
