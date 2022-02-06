use actix_web::HttpResponse;
use serde::Serialize;

#[doc(hidden)]
#[derive(Serialize)]
pub struct ApiInfoResponse<'a> {
    name: &'a str,
    version: &'a str,
    authors: Vec<&'a str>,
    license: &'a str,
}

pub async fn api_info() -> HttpResponse {
    let response_body = ApiInfoResponse {
        name: env!("CARGO_PKG_NAME"),
        version: env!("CARGO_PKG_VERSION"),
        authors: env!("CARGO_PKG_AUTHORS").split(':').collect(),
        license: env!("CARGO_PKG_LICENSE"),
    };
    HttpResponse::Ok().json(response_body)
}
