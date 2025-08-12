mod auth;
mod judge;
mod repository;
mod service;

use crate::common::response::ApiResponse;
use axum::{Json, http::StatusCode, response::IntoResponse};
use thiserror::Error;

pub use auth::AuthError;
pub use judge::JudgeError;
pub use repository::RepositoryError;
pub use service::ServiceError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Internal server error")]
    InternalError,
    #[error("Failed to initialize tokio runtime: {0}")]
    InitializationError(#[from] std::io::Error),
    #[error("Database connection error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),
    #[error("Application setup error: {0}")]
    Setup(String),
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
        let (status_code, error_message, error_code) = match self {
            Error::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
                "INTERNAL_ERROR",
            ),
            Error::InitializationError(ref e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to initialize: {}", e),
                "INITIALIZATION_ERROR",
            ),
            Error::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to establish database connection".to_string(),
                "DATABASE_ERROR",
            ),
            Error::Setup(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                msg.clone(),
                "SETUP_ERROR",
            ),
            Error::Repository(ref repo_err) => match repo_err {
                RepositoryError::NotFound => (
                    StatusCode::NOT_FOUND,
                    "Resource not found".to_string(),
                    "NOT_FOUND",
                ),
                RepositoryError::UniqueViolation(msg) => {
                    (StatusCode::CONFLICT, msg.clone(), "UNIQUE_VIOLATION")
                }
                RepositoryError::ForeignKeyViolation(msg) => (
                    StatusCode::BAD_REQUEST,
                    msg.clone(),
                    "FOREIGN_KEY_VIOLATION",
                ),
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Repository error occurred".to_string(),
                    "REPOSITORY_ERROR",
                ),
            },
            Error::Service(service_err) => match service_err {
                ServiceError::ValidationError(ref msg) => {
                    (StatusCode::BAD_REQUEST, msg.clone(), "VALIDATION_ERROR")
                }
                ServiceError::NotFoundError(msg) => {
                    (StatusCode::NOT_FOUND, msg.clone(), "NOT_FOUND")
                }
                ServiceError::ConflictError(msg) => (StatusCode::CONFLICT, msg.clone(), "CONFLICT"),
                ServiceError::UnauthorizedError(msg) => {
                    (StatusCode::UNAUTHORIZED, msg.clone(), "UNAUTHORIZED")
                }
                ServiceError::ForbiddenError(msg) => {
                    (StatusCode::FORBIDDEN, msg.clone(), "FORBIDDEN")
                }
                ServiceError::RateLimitError => (
                    StatusCode::TOO_MANY_REQUESTS,
                    "Rate limit exceeded".to_string(),
                    "RATE_LIMIT_EXCEEDED",
                ),
                ServiceError::TimeoutError => (
                    StatusCode::REQUEST_TIMEOUT,
                    "Request timeout".to_string(),
                    "TIMEOUT",
                ),
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Service error occurred".to_string(),
                    "SERVICE_ERROR",
                ),
            },
            Error::Auth(ref auth_err) => match auth_err {
                AuthError::InvalidCredentials => (
                    StatusCode::UNAUTHORIZED,
                    "Invalid credentials".to_string(),
                    "INVALID_CREDENTIALS",
                ),
                AuthError::TokenExpired => (
                    StatusCode::UNAUTHORIZED,
                    "Token has expired".to_string(),
                    "TOKEN_EXPIRED",
                ),
                AuthError::InvalidToken(_) => (
                    StatusCode::UNAUTHORIZED,
                    "Invalid token".to_string(),
                    "INVALID_TOKEN",
                ),
                AuthError::MissingToken => (
                    StatusCode::UNAUTHORIZED,
                    "Authentication token required".to_string(),
                    "MISSING_TOKEN",
                ),
                AuthError::JwtError(_) => (
                    StatusCode::UNAUTHORIZED,
                    "Authentication error".to_string(),
                    "JWT_ERROR",
                ),
            },
            Error::Judge(ref judge_err) => match judge_err {
                JudgeError::CompilationError(msg) => (
                    StatusCode::BAD_REQUEST,
                    format!("Compilation failed: {}", msg),
                    "COMPILATION_ERROR",
                ),
                JudgeError::RuntimeError(msg) => (
                    StatusCode::BAD_REQUEST,
                    format!("Runtime error: {}", msg),
                    "RUNTIME_ERROR",
                ),
                JudgeError::TimeLimitExceeded => (
                    StatusCode::BAD_REQUEST,
                    "Time limit exceeded".to_string(),
                    "TIME_LIMIT_EXCEEDED",
                ),
                JudgeError::MemoryLimitExceeded => (
                    StatusCode::BAD_REQUEST,
                    "Memory limit exceeded".to_string(),
                    "MEMORY_LIMIT_EXCEEDED",
                ),
                JudgeError::WrongAnswer => (
                    StatusCode::BAD_REQUEST,
                    "Wrong answer".to_string(),
                    "WRONG_ANSWER",
                ),
                JudgeError::UnsupportedLanguage(lang) => (
                    StatusCode::BAD_REQUEST,
                    format!("Unsupported language: {}", lang),
                    "UNSUPPORTED_LANGUAGE",
                ),
                _ => (
                    StatusCode::BAD_REQUEST,
                    "Judge error occurred".to_string(),
                    "JUDGE_ERROR",
                ),
            },
            Error::Validation(ref msg) => {
                (StatusCode::BAD_REQUEST, msg.clone(), "VALIDATION_ERROR")
            }
            Error::NotFound(ref msg) => (StatusCode::NOT_FOUND, msg.clone(), "NOT_FOUND"),
            Error::BadRequest(ref msg) => (StatusCode::BAD_REQUEST, msg.clone(), "BAD_REQUEST"),
        };

        let api_response = ApiResponse::error_with_code(error_message, error_code.to_string());
        (status_code, Json(api_response)).into_response()
    }
}

pub type Result<T> = std::result::Result<T, Error>;
