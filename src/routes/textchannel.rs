use actix_web::web::Json;
use actix_web::{web, HttpResponse};
use log::info;
use serde::Deserialize;
use uuid::Uuid;

use crate::models::error::AzumaError;
use crate::models::session::Session;
use crate::models::textchannel::TextChannel;
use crate::AzumaState;

#[derive(Deserialize)]
pub struct TextchannelCreateRequest {
    name: String,
    description: Option<String>,
}

pub async fn create_textchannel(
    req: Json<TextchannelCreateRequest>,
    state: web::Data<AzumaState>,
    session: Session,
) -> Result<HttpResponse, AzumaError> {
    // Clean up false input which could screw up the database
    let description = req
        .description
        .as_deref()
        .map(|x| if x.trim().len() == 0 { None } else { Some(x) })
        .flatten();

    let textchannel = TextChannel::new(&state.db, &req.name, description).await?;
    info!(target: "REST API", "User '{user}' created TextChannel with name '{name}'", user = session.subject, name = req.name);
    Ok(HttpResponse::Created().json(textchannel))
}

#[derive(Deserialize)]
pub struct TextchannelDeleteRequest {
    pub id: Uuid,
}

//TODO: proper authorization
pub async fn delete_textchannel(
    req: Json<TextchannelDeleteRequest>,
    session: Session,
    state: web::Data<AzumaState>,
) -> Result<HttpResponse, AzumaError> {
    TextChannel::remove(&state.db, &req.id).await?;
    Ok(HttpResponse::NoContent().finish())
}