use crate::{websocket::connection::Ws, AzumaState};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use log::info;
use uuid::Uuid;

/// This route responds to clients wanting to upgrade their connection to a websocket
pub(crate) async fn init_ws(
    data: web::Data<AzumaState>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let resp = ws::start(
        Ws {
            data,
            subject: None,
            connection_id: Uuid::new_v4(),
        },
        &req,
        stream,
    );
    info!(target: "REST API", "Upgrading connection to websocket from {}:{}", req.peer_addr().unwrap().ip(), req.peer_addr().unwrap().port());
    resp
}
