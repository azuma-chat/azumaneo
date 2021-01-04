use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use argon2::Error as Argon2Error;
use serde::Serialize;
use sqlx::{postgres::PgDatabaseError, Error as SqlxError};
use std::error::Error as ErrorTrait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AzumaError {
    #[error("ALREADY_EXISTS")]
    AlreadyExists,
    #[error("FORBIDDEN")]
    Forbidden,
    #[error("INTERNAL_SERVER_ERROR")]
    InternalServerError { source: Box<dyn ErrorTrait> },
    #[error("NOT_FOUND")]
    NotFound,
    #[error("UNAUTHORIZED")]
    Unauthorized,
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
            Forbidden => StatusCode::FORBIDDEN,
            InternalServerError { source: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            NotFound => StatusCode::NOT_FOUND,
            Unauthorized => StatusCode::UNAUTHORIZED,
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
        // 23505 conflict
        if let SqlxError::Database(err) = &err {
            let err = err.downcast_ref::<PgDatabaseError>();
            match err.code() {
                "23505" => return AzumaError::AlreadyExists, // unique_violation
                _ => (),
            }
        }
        AzumaError::InternalServerError {
            source: Box::new(err),
        }
    }
}
