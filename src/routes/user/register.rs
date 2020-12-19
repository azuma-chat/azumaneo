use crate::models::rejection::handle_rejection;
use crate::models::user::User;
use crate::util::get_header_value_simple;
use actix_web::body::Body;
use actix_web::{HttpRequest, HttpResponse};

pub async fn register_user(req: HttpRequest) -> HttpResponse {
    //Get header values, if not present, return error
    let username = match get_header_value_simple(&req, "name") {
        Ok(name) => name,
        Err(err) => return err,
    };

    let passwd = match get_header_value_simple(&req, "password") {
        Ok(passwd) => passwd,
        Err(err) => return err,
    };

    let user = match User::new(username, passwd).await {
        Ok(user) => user,
        Err(err) => return handle_rejection(&req, err),
    };

    HttpResponse::Ok().body(Body::from(format!("{:?}", user)))
}
