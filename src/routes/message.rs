use actix_web::{web, HttpResponse};
use log::info;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::error::AzumaError;
use crate::models::message::ChatMessage;
use crate::models::session::Session;
use crate::AzumaState;

#[doc(hidden)]
#[derive(Deserialize, Clone)]
pub struct SendMessageRequest {
    channel: Uuid,
    content: String,
}

#[doc(hidden)]
#[derive(Serialize)]
pub struct SendMessageResponse {
    id: Uuid,
}

/// This routes purpose is to send a chat message in the name of a user
pub async fn send_msg(
    state: web::Data<AzumaState>,
    request: web::Json<SendMessageRequest>,
    session: Session,
) -> Result<HttpResponse, AzumaError> {
    info!(target: "REST API", "ChatMessage sent in '{channel}' by '{user}'", channel = request.channel, user = session.subject);
    ChatMessage::new(
        &session.subject,
        &request.channel,
        &request.content,
        &state.broker,
        &state.db,
    )
    .await?;

    Ok(HttpResponse::Ok().finish())
}
