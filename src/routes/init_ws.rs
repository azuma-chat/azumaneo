use crate::websocket::handler::Ws;
use crate::AzumaState;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use uuid::Uuid;

pub(crate) async fn init_ws(
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<AzumaState>,
) -> Result<HttpResponse, Error> {
    let resp = ws::start(
        Ws {
            session_id: Uuid::new_v4(),
            user_id: None,
            state,
        },
        &req,
        stream,
    );
    println!("upgrading connection from {:?}", req.peer_addr());
    resp
}
