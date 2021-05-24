use crate::models::session::Session;
use crate::models::{error::AzumaError, message::ChatMessage};
use crate::websocket::broker::Broadcast;
use crate::AzumaState;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    state.broker.do_send(Broadcast::ChatMessage(ChatMessage {
        id: Uuid::new_v4(),
        authorid: session.subject,
        channelid: request.channel.clone(),
        content: request.content.clone(),
        created_at: Utc::now(),
    }));

    Ok(HttpResponse::Ok().finish())
}
