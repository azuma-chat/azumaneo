use actix_web::{HttpRequest, HttpResponse};
use crate::placeholder_route;

pub fn api_info(req: HttpRequest) -> HttpResponse{

    //TODO: Implement '/api/info' route

    placeholder_route(req)
}