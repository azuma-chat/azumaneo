use argon2::Error as Argon2Error;
use sqlx::Error as SqlxError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AzumaError {
    #[error("this object already exists")]
    AlreadyExists,
    #[error("an internal server error happened")]
    InternalServerError,
}

impl From<Argon2Error> for AzumaError {
    fn from(_: Argon2Error) -> Self {
        AzumaError::InternalServerError
    }
}

impl From<SqlxError> for AzumaError {
    fn from(_: SqlxError) -> Self {
        AzumaError::InternalServerError
    }
}
