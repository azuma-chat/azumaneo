use serde::Serialize;
use actix_web::{HttpRequest, HttpResponse, body::Body};
use actix_web::http::StatusCode;

#[allow(dead_code)]
#[derive(Debug)]
pub enum AzumaRejection {
    AlreadyExists,
    InternalServerError,
    NotFound,
    Unauthorized,
    BadRequest
}

impl PartialEq for AzumaRejection {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Serialize)]
pub struct RejectionMessage {
    code: i32,
    message: String,
}

#[allow(unused_variables)]
pub fn handle_rejection(req: &HttpRequest, cause: AzumaRejection) -> HttpResponse {
    let (code, message) = match cause {
        AzumaRejection::NotFound => {
            (StatusCode::NOT_FOUND.as_u16(), "NOT_FOUND")
        },
        AzumaRejection::AlreadyExists => {
            (StatusCode::BAD_REQUEST.as_u16(), "ALREADY_EXISTS")
        },
        AzumaRejection::InternalServerError => {
            (StatusCode::INTERNAL_SERVER_ERROR.as_u16(), "INTERNAL_SERVER_ERROR")
        },
        AzumaRejection::Unauthorized => {
            (StatusCode::UNAUTHORIZED.as_u16(), "UNAUTHORIZED")
        },
        AzumaRejection::BadRequest => {
            (StatusCode::BAD_REQUEST.as_u16(), "BAD_REQUEST")
        }
    };

    let json = serde_json::to_string(&RejectionMessage {
        code: code as i32,
        message: message.to_string(),
    }).expect("failed parsing json");
    HttpResponse::new(StatusCode::from_u16(code).expect("failed parsing the statuscode")).set_body(Body::from(json))
}