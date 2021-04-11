use crate::models::error::AzumaError;
use crate::models::message::ChatMessage;
use crate::models::session::Session;
use crate::websocket::channelhandler::MessageSendRequest;
use crate::AzumaState;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Clone)]
pub struct SendMessageRequest {
    token: Uuid,
    channel_id: Uuid,
    content: String,
}

#[derive(Serialize)]
pub struct SendMessageResponse {
    id: Uuid,
}

pub async fn send_msg(
    send_request: web::Json<SendMessageRequest>,
    state: web::Data<AzumaState>,
) -> Result<HttpResponse, AzumaError> {
    let userid = Session::get_by_token(&send_request.token, &state.db)
        .await?
        .subject;
    state
        .channelhandler
        .do_send(MessageSendRequest(ChatMessage {
            id: Uuid::new_v4(),
            author: userid,
            channel: send_request.channel_id.clone(),
            content: send_request.content.clone(),
            timestamp: Utc::now(),
        }));

    Ok(HttpResponse::Ok().finish())
}
