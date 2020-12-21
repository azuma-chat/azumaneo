use crate::models::etc::DefaultResponse;
use actix_web::{body::Body, HttpRequest, HttpResponse};

pub fn get_header_value_simple(
    req: &HttpRequest,
    header_name: &str,
) -> Result<String, HttpResponse> {
    let req_error = DefaultResponse {
        code: 400,
        message: format!("Bad request. Header '{}' is missing.", &header_name),
    };
    let conv_error = DefaultResponse {
        code: 400,
        message: format!(
            "Bad request. Header '{}' couldn't be converted to str.",
            &header_name
        ),
    };
    let err_response = HttpResponse::BadRequest()
        .header("Content-Type", "application/json")
        .finish();
    let headers = req.headers();
    let header = match headers.get(header_name) {
        Some(header) => match header.to_str() {
            Ok(header) => header,
            Err(_) => {
                return Err(
                    err_response.set_body(Body::from(serde_json::to_string(&conv_error).unwrap()))
                )
            }
        },
        None => {
            return Err(
                err_response.set_body(Body::from(serde_json::to_string(&req_error).unwrap()))
            )
        }
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
        None => "None".to_string(),
    }
}

#[allow(unused_must_use)]
pub fn print_console_err(input: String) {
    let mut terminal = term::stdout().expect("Failed getting the terminal!");
    terminal.fg(term::color::BRIGHT_RED).unwrap();
    terminal.attr(term::Attr::Bold).unwrap();
    println!("{}", input);
    terminal.reset();
}
