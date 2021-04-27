use crate::models::awsp::etc::OnlineStatus;
use crate::models::session::Session;
use crate::AzumaState;
use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[doc(hidden)]
#[derive(Deserialize)]
pub struct UpdateOnlinestatusRequest {
    pub status: OnlineStatus,
}
/// This route is used to update a users own onlinestatus
pub async fn update_onlinestatus(
    _req: web::Json<UpdateOnlinestatusRequest>,
    _session: Session,
    _state: web::Data<AzumaState>,
) -> HttpResponse {
    /*state.srv.do_send(UpdateUserOnlinestatus {
        user: session.subject,
        status: *req.status,
    });*/
    // TODO: set online status here
    HttpResponse::Ok().finish()
}
