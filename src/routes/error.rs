use crate::user::CreateUserError;
use actix_web::ResponseError;
use actix_web::http::StatusCode;

// #[derive(thiserror::Error, Debug)]
// pub enum UserCreationError {
//     #[error("{0}")]
//     ValidationError(String),
//     // delegamos transparentemente ambos `Display`s y la implementacion de `source`s al type
//     // wrappeado por `UnexpectedError`
//     #[error(transparent)]
//     UnexpectedError(#[from] anyhow::Error),
// }

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

// /// Retornamos un opaco 500 mientras preservamos la causa principal del error para hacer logging
// pub fn e500<T>(e: T) -> actix_web::Error
// where
//     T: std::fmt::Debug + std::fmt::Display + 'static,
// {
//     actix_web::error::ErrorInternalServerError(e)
// }
//
// pub fn see_other(location: &str) -> HttpResponse {
//     HttpResponse::SeeOther()
//         .insert_header((LOCATION, location))
//         .finish()
// }
//
// /// Retornamos un 400 con la representacion del usuario para el error como body
// /// donde la causa del error principal se preserva para loggin
// pub fn e400<T>(e: T) -> actix_web::Error
// where
//     T: std::fmt::Debug + std::fmt::Display + 'static,
// {
//     actix_web::error::ErrorBadRequest(e)
// }
