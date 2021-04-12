use serde::Serialize;
use actix::Message;
use std::collections::HashMap;
use crate::models::awsp::error::AwspErrorType::BadRequest;
use crate::models::awsp::wrapper::{AwspWrapper, AwspMsgType};
use crate::AzumaState;

#[derive(Serialize, Message)]
#[rtype(response = "()")]
pub struct AwspError {
    pub errortype: AwspErrorType,
}

#[derive(Serialize)]
pub enum AwspErrorType {
    BadRequest,
    Unauthorized,
    AlreadyExists,
    InternalServerError,
}
impl AwspErrorType {
    pub fn into_hm(&self) -> HashMap<String, String> {
        let mut hm: HashMap<String, String> = HashMap::new();
        match self {
            AwspErrorType::BadRequest => {hm.insert("errortype".to_string(), "BadRequest".to_string());}
            AwspErrorType::Unauthorized => {hm.insert("errortype".to_string(), "Unauthorized".to_string());}
            AwspErrorType::AlreadyExists => {hm.insert("errortype".to_string(), "AlreadyExists".to_string());}
            AwspErrorType::InternalServerError => {hm.insert("errortype".to_string(), "InternalServerError".to_string());}
        }
        hm
    }

    pub fn into_wrapper(&self, state: &AzumaState) -> AwspWrapper {
        AwspWrapper {
            version: state.constants.awsp_version.to_string(),
            msg_type: AwspMsgType::Error,
            content: self.into_hm()
        }
    }
}