use actix_web::web;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::error::AzumaError;
use crate::models::textchannel::TextChannel;
use crate::AzumaState;

#[derive(Deserialize)]
pub struct TextchannelCreateRequest<'a> {
    name: String,
    description: Option<&'a str>,
}

#[derive(Serialize)]
pub struct TextchannelCreateResponse {
    id: Uuid,
    name: String,
    description: Option<String>,
}

pub async fn create_textchannel(
    mut req: TextchannelCreateRequest<'_>,
    state: web::Data<AzumaState>,
) -> Result<TextchannelCreateResponse, AzumaError> {
    // Clean up false input which screw up the database
    if let Some("") = req.description {
        req.description = None;
    }
    // Change the inner value of the option in order to be able to pass it on
    let description = match req.description {
        None => None,
        Some(str) => Some(str.to_string()),
    };
    let txchannel = TextChannel::new(&state.db, req.name, description).await?;
    Ok(TextchannelCreateResponse {
        id: txchannel.id,
        name: txchannel.name,
        description: txchannel.description,
    })
}
