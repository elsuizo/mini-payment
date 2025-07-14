use crate::user::{CreateUserError, DatabaseError};
use actix_web::ResponseError;
use actix_web::http::StatusCode;

impl ResponseError for CreateUserError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Self::InvalidName(_) => StatusCode::BAD_REQUEST,
            Self::InvalidCountryName(_) => StatusCode::BAD_REQUEST,
            Self::InvalidDocumentNumber(_) => StatusCode::BAD_REQUEST,
            Self::UserAlreadyExistsError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl ResponseError for DatabaseError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Self::UnknownUser(_) => StatusCode::BAD_REQUEST,
            Self::InsufficientBalance(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
