use crate::models::awsp::etc::OnlineStatus;
use crate::models::session::Session;
use crate::websocket::chatserver::UpdateUserOnlinestatus;
use crate::AzumaState;
use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateOnlinestatusRequest {
    pub status: OnlineStatus,
}

pub async fn update_onlinestatus(
    req: web::Json<UpdateOnlinestatusRequest>,
    session: Session,
    state: web::Data<AzumaState>,
) -> HttpResponse {
    state.srv.do_send(UpdateUserOnlinestatus {
        user: session.subject,
        status: *req.status,
    });
    HttpResponse::Ok().finish()
}
