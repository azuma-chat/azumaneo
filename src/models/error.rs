use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use argon2::Error as Argon2Error;
use serde::Serialize;
use sqlx::Error as SqlxError;
use std::error::Error as ErrorTrait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AzumaError {
    #[error("ALREADY_EXISTS")]
    AlreadyExists,
    #[error("INTERNAL_SERVER_ERROR")]
    InternalServerError { source: Box<dyn ErrorTrait> },
}

#[derive(Serialize)]
struct ResponseBody {
    message: String,
}

impl ResponseError for AzumaError {
    fn error_response(&self) -> HttpResponse {
        let response_body = ResponseBody {
            message: format!("{}", self),
        };
        HttpResponse::build(self.status_code()).json(response_body)
    }

    fn status_code(&self) -> StatusCode {
        use AzumaError::*;
        match self {
            AlreadyExists => StatusCode::CONFLICT,
            InternalServerError { source: _ } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<Argon2Error> for AzumaError {
    fn from(err: Argon2Error) -> Self {
        AzumaError::InternalServerError {
            source: Box::new(err),
        }
    }
}

impl From<SqlxError> for AzumaError {
    fn from(err: SqlxError) -> Self {
        AzumaError::InternalServerError {
            source: Box::new(err),
        }
    }
}
