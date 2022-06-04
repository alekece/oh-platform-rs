use diesel::result::{DatabaseErrorKind, Error as DieselError};
use rocket::http::Status;
use rocket::serde::json::Error as JsonError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    BadRequest(String),
    #[error("Conflicted data : {0}")]
    ConflictedData(String),
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("Resource not found")]
    NotFound,
    #[error("Unknown route: {0}")]
    UnknownRoute(String),
    #[error("Internal error: {0:?}")]
    InternalError(String),
}

impl Error {
    pub fn get_http_status(&self) -> Status {
        match *self {
            Self::InvalidData(_) => Status::UnprocessableEntity,
            Self::ConflictedData(_) => Status::Conflict,
            Self::BadRequest(_) => Status::BadRequest,
            Self::NotFound | Self::UnknownRoute(_) => Status::NotFound,
            _ => Status::InternalServerError,
        }
    }
}

impl From<JsonError<'_>> for Error {
    fn from(e: JsonError<'_>) -> Self {
        match e {
            JsonError::Io(e) => Self::BadRequest(format!("Cannot process JSON: {}", e)),
            JsonError::Parse(data, e) => Self::BadRequest(format!("Invalid JSON format '{}': {}", data, e)),
        }
    }
}

impl From<DieselError> for Error {
    fn from(e: DieselError) -> Self {
        match e {
            DieselError::NotFound => Self::NotFound,
            DieselError::DatabaseError(
                DatabaseErrorKind::UniqueViolation | DatabaseErrorKind::ForeignKeyViolation,
                e,
            ) => Self::ConflictedData(e.message().to_string()),
            e => Self::InternalError(e.to_string()),
        }
    }
}
