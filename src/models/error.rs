use actix::{MailboxError, Message};
use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use sqlx::{postgres::PgDatabaseError, Error as SqlxError};
use std::error::Error as ErrorTrait;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Argon2id returned an error")]
pub struct Argon2idError;

/// The AzumaError is used if an error occures somewhere. Its used internally the same as its used in communication with the clients.
#[derive(Debug, Error, Message)]
#[rtype(response = "()")]
pub enum AzumaError {
    #[error("ALREADY_EXISTS")]
    AlreadyExists,
    #[error("BAD_REQUEST")]
    BadRequest,
    #[error("FORBIDDEN")]
    Forbidden,
    #[error("INTERNAL_SERVER_ERROR")]
    InternalServerError { source: Box<dyn ErrorTrait> },
    #[error("NOT_FOUND")]
    NotFound,
    #[error("UNAUTHORIZED")]
    Unauthorized,
}

/// This is a helper struct we need in order to be able to return AzumaError from functions without building a http response first
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

    /// Map http statuscodes to the corresponding [`AzumaError`] variants
    fn status_code(&self) -> StatusCode {
        use AzumaError::*;
        match self {
            AlreadyExists => StatusCode::CONFLICT,
            BadRequest => StatusCode::BAD_REQUEST,
            Forbidden => StatusCode::FORBIDDEN,
            InternalServerError { source: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            NotFound => StatusCode::NOT_FOUND,
            Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }
}

impl From<Argon2idError> for AzumaError {
    fn from(err: Argon2idError) -> Self {
        AzumaError::InternalServerError {
            source: Box::new(err),
        }
    }
}

impl From<MailboxError> for AzumaError {
    fn from(err: MailboxError) -> Self {
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
            if let "23505" = err.code() {
                return AzumaError::AlreadyExists;
            }
        }
        if let SqlxError::RowNotFound = &err {
            return AzumaError::NotFound;
        }

        AzumaError::InternalServerError {
            source: Box::new(err),
        }
    }
}
