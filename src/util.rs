use mongodb::{Database, Client};
use std::env;
use crate::models::etc::DefaultResponse;
use actix_web::{HttpRequest, HttpResponse, body::Body};


//TODO: Error message when db connection couldN't be established
pub async fn get_mongodb() -> Database {
    let db_client = Client::with_uri_str(
        &env::var("AZUMA_MONGODB").expect("Environment variable AZUMA_MONGODB not found"),
    ).await.expect("Error creating MongoDB client");
    db_client.database(
        &env::var("AZUMA_DBNAME").expect("Environment variable AZUMA_DBNAME not found"),
    )
}

pub fn get_header_value_simple(req: &HttpRequest, header_name: &str) -> Result<String, HttpResponse> {
    let req_error = DefaultResponse {
        code: 400,
        message: format!("Bad request. Header '{}' is missing.", &header_name),
    };
    let conv_error = DefaultResponse {
        code: 400,
        message: format!("Bad request. Header '{}' couldn't be converted to str.", &header_name),
    };
    let err_response = HttpResponse::BadRequest()
        .header("Content-Type", "application/json")
        .finish();
    let headers = req.headers();
    let header = match headers.get(header_name) {
        Some(header) => match header.to_str() {
            Ok(header) => header,
            Err(_) => return Err(err_response.set_body(Body::from(serde_json::to_string(&conv_error).unwrap())))
        },
        None => return Err(err_response.set_body(Body::from(serde_json::to_string(&req_error).unwrap()))),
    };

    Ok(header.parse().unwrap())
}


pub fn get_header_value(req: &HttpRequest, header_name: &str) -> Option<String> {
    let headers = req.headers();
    match headers.get(header_name) {
        Some(header) => match header.to_str() {
            Ok(header) => Some(header.to_string()),
            Err(_) => None,
        },
        None => None,
    }
}

pub fn prettyprint_option_string(opt: Option<String>) -> String {
    match opt {
        Some(string) => string,
        None => "None".to_string()
    }
}