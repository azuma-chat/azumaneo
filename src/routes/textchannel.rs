use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::error::AzumaError;
use crate::models::session::Session;
use crate::models::textchannel::TextChannel;
use crate::AzumaState;
use actix_web::web::Json;

#[derive(Deserialize)]
pub struct TextchannelCreateRequest {
    name: String,
    description: Option<String>,
}

#[derive(Serialize)]
pub struct TextchannelCreateResponse {
    id: Uuid,
    name: String,
    description: Option<String>,
}

pub async fn create_textchannel(
    mut req: Json<TextchannelCreateRequest>,
    state: web::Data<AzumaState>,
    session: Session,
) -> Result<HttpResponse, AzumaError> {
    // Clean up false input which screw up the database
    if req.description == Some("".to_string()) {
        req.description = None;
    }
    // Change the inner value of the option in order to be able to pass it on
    let description = match &req.description {
        None => None,
        Some(str) => Some(str.to_string()),
    };
    let txchannel = TextChannel::new(&state.db, req.name.clone(), description).await?;
    let response = TextchannelCreateResponse {
        id: txchannel.id,
        name: txchannel.name,
        description: txchannel.description,
    };
    Ok(HttpResponse::Ok().json(response))
}
