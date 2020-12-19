use crate::placeholder_route;
use actix_web::{HttpRequest, HttpResponse};

pub fn login_user(req: HttpRequest) -> HttpResponse {
    //TODO: Implement '/user/login' route

    placeholder_route(req)
}
