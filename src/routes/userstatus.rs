use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use serde::Deserialize;

use crate::{
    models::{
        error::AzumaError,
        session::Session,
        stateactor::{OnlineStatus, SetOnlineStatus},
    },
    AzumaState,
};

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
