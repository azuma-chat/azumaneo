use crate::{websocket::connection::Ws, AzumaState};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

/// This route responds to clients wanting to upgrade their connection to a websocket
pub(crate) async fn init_ws(
    data: web::Data<AzumaState>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let resp = ws::start(Ws { data }, &req, stream);
    println!(
        "upgrading connection from {}:{}",
        req.peer_addr().unwrap().ip(),
        req.peer_addr().unwrap().port()
    );
    resp
}
