use serde::Serialize;
#[derive(Serialize)]
pub struct DefaultResponse {
    pub(crate) code: u32,
    pub(crate) message: String,
}