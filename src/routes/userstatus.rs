use crate::models::error::AzumaError;
use crate::models::session::Session;
use crate::models::stateactor::{OnlineStatus, SetOnlineStatus};
use crate::AzumaState;
use actix_web::web::{Data, Json};
use actix_web::HttpResponse;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct StatusSetRequest {
    pub status: OnlineStatus,
}

pub async fn set_onlinestatus(
    request: Json<StatusSetRequest>,
    state: Data<AzumaState>,
    session: Session,
) -> Result<HttpResponse, AzumaError> {
    match state
        .state
        .send(SetOnlineStatus {
            user: session.subject,
            status: request.status,
        })
        .await?
    {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(err) => Err(err),
    }
}
