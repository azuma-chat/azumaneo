use std::str::FromStr;

use actix_web::web::{self};
use actix_web::{HttpRequest, HttpResponse};
use chrono::{DateTime, Utc};
use log::info;
use serde::{Deserialize, Serialize};
use sodiumoxide::crypto::pwhash::argon2id13::{self, HashedPassword};
use uuid::Uuid;

use crate::models::error::{Argon2idError, AzumaError};
use crate::models::session::Session;
use crate::models::stateactor::{GetOnlineStatus, OnlineStatus};
use crate::models::user::User;
use crate::AzumaState;

#[doc(hidden)]
#[derive(Deserialize)]
pub struct RegisterUserRequest {
    name: String,
    password: String,
}

#[doc(hidden)]
#[derive(Serialize)]
pub struct RegisterUserResponse {
    token: Uuid,
}
/// Register a given user to the azuma database
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

#[doc(hidden)]
#[derive(Deserialize)]
pub struct LoginUserRequest {
    name: String,
    password: String,
}

#[doc(hidden)]
#[derive(Serialize)]
pub struct LoginUserResponse {
    token: Uuid,
}

/// Try to login a user and respond with a valid session token
pub async fn login_user(
    data: web::Data<AzumaState>,
    request: web::Json<LoginUserRequest>,
    req: HttpRequest,
) -> Result<HttpResponse, AzumaError> {
    let user = User::get_by_name(&request.name, &data.db).await?;
    let hashed_password = HashedPassword::from_slice(&user.password).ok_or(Argon2idError)?;

    if argon2id13::pwhash_verify(&hashed_password, request.password.as_bytes()) {
        let session = Session::new(&user, &data.db).await?;
        info!(target: "Access Control", "User '{}' logged in from '{}'", session.subject, req.connection_info().realip_remote_addr().unwrap_or("None"));
        let response_body = LoginUserResponse {
            token: session.token,
        };
        Ok(HttpResponse::Ok().json(response_body))
    } else {
        Err(AzumaError::Forbidden)
    }
}

#[doc(hidden)]
#[derive(Deserialize)]
pub struct UpdateUserRequest {
    name: Option<String>,
    password: Option<String>,
}

#[doc(hidden)]
#[derive(Serialize)]
pub struct UpdateUserResponse {
    id: Uuid,
    name: String,
    created_at: DateTime<Utc>,
}

/// Update a users details
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
    info!(target: "Access Control", "User '{}' was updated", user.id);
    let response_body = UpdateUserResponse {
        id: user.id,
        name: user.name,
        created_at: user.created_at,
    };
    Ok(HttpResponse::Ok().json(response_body))
}

#[derive(Deserialize)]
pub struct FetchUserRequest {
    pub user: Uuid,
}

#[derive(Serialize)]
pub struct FetchUserResponse {
    pub onlinestatus: OnlineStatus,
    // apply flatten in order to pull the fields out of the user struct
    #[serde(flatten)]
    pub user: User,
}

/// Fetch information about a user
pub async fn fetch_user(
    path: web::Path<String>,
    state: web::Data<AzumaState>,
    session: Session,
) -> Result<HttpResponse, AzumaError> {
    let user = path.into_inner();

    let user_id = {
        if &user == "self" {
            session.subject
        } else {
            Uuid::from_str(&user)?
        }
    };

    let user = User::get_by_id(&user_id, &state.db).await?;
    let onlinestatus = state.state.send(GetOnlineStatus { user: user_id }).await?;
    let response = FetchUserResponse {
        onlinestatus: onlinestatus,
        user: user,
    };

    Ok(HttpResponse::Ok().json(response))
}
