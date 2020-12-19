use crate::placeholder_route;
use actix_web::{HttpRequest, HttpResponse};

pub fn api_info(req: HttpRequest) -> HttpResponse {
    //TODO: Implement '/api/info' route

    placeholder_route(req)
}
