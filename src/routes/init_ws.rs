use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_web_actors::ws;
use crate::AzumaState;
use crate::websocket::handler::Ws;

pub(crate) async fn init_ws(req: HttpRequest, stream: web::Payload, state: web::Data<AzumaState>) -> Result<HttpResponse, Error> {
    let resp = ws::start(Ws {
        session_id: Default::default(),
        user_id: None,
        state,
    }, &req, stream);
    println!("upgrading connection from {:?}", req.peer_addr());
    resp
}