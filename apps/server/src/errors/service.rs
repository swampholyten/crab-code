use super::RepositoryError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Not found: {0}")]
    NotFoundError(String),

    #[error("Conflict: {0}")]
    ConflictError(String),

    #[error("Unauthorized: {0}")]
    UnauthorizedError(String),

    #[error("Forbidden: {0}")]
    ForbiddenError(String),

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("External service error: {0}")]
    ExternalServiceError(String),

    #[error("Rate limit exceeded")]
    RateLimitError,

    #[error("Execution timeout")]
    TimeoutError,
}
