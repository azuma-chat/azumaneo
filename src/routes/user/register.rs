use actix_web::{HttpRequest, HttpResponse};
use crate::placeholder_route;

pub fn register_user(req: HttpRequest) -> HttpResponse{

    //TODO: Implement '/user/register' route

    placeholder_route(req)
}