mod auth;
mod judge;
mod repository;
mod service;

use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

pub use auth::AuthError;
pub use judge::JudgeError;
pub use repository::RepositoryError;
pub use service::ServiceError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Internal server error")]
    InternalError,

    #[error("Service error: {0}")]
    Service(#[from] ServiceError),

    #[error("Auth error: {0}")]
    Auth(#[from] AuthError),

    #[error("Judge error: {0}")]
    Judge(#[from] JudgeError),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let status_code = match self {
            Error::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Service(ref service_err) => match service_err {
                ServiceError::ValidationError(_) => StatusCode::BAD_REQUEST,
                ServiceError::NotFoundError(_) => StatusCode::NOT_FOUND,
                ServiceError::ConflictError(_) => StatusCode::CONFLICT,
                ServiceError::UnauthorizedError(_) => StatusCode::UNAUTHORIZED,
                ServiceError::ForbiddenError(_) => StatusCode::FORBIDDEN,
                ServiceError::RateLimitError => StatusCode::TOO_MANY_REQUESTS,
                ServiceError::TimeoutError => StatusCode::REQUEST_TIMEOUT,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            Error::Auth(ref auth_err) => match auth_err {
                AuthError::InvalidCredentials => StatusCode::UNAUTHORIZED,
                AuthError::TokenExpired => StatusCode::UNAUTHORIZED,
                AuthError::InvalidToken(_) => StatusCode::UNAUTHORIZED,
                AuthError::MissingToken => StatusCode::UNAUTHORIZED,
                AuthError::JwtError(_) => StatusCode::UNAUTHORIZED,
            },
            Error::Judge(_) => StatusCode::BAD_REQUEST,
            Error::Validation(_) => StatusCode::BAD_REQUEST,
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
        };
        (status_code, self.to_string()).into_response()
    }
}

pub type Result<T> = std::result::Result<T, Error>;
