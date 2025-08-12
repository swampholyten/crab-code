use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Entity not found")]
    NotFound,

    #[error("Unique constraint violation: {0}")]
    UniqueViolation(String),

    #[error("Foreign key constraint violation: {0}")]
    ForeignKeyViolation(String),

    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Transaction error: {0}")]
    Transaction(String),

    #[error("Serialization error: {0}")]
    Serialization(String),
}

impl From<sqlx::Error> for RepositoryError {
    fn from(err: sqlx::Error) -> Self {
        match &err {
            sqlx::Error::RowNotFound => RepositoryError::NotFound,
            sqlx::Error::Database(db_err) => {
                let code = db_err.code().unwrap_or_default();
                match code.as_ref() {
                    "23505" => RepositoryError::UniqueViolation(db_err.message().to_string()),
                    "23503" => RepositoryError::ForeignKeyViolation(db_err.message().to_string()),
                    _ => RepositoryError::Database(db_err.message().to_string()),
                }
            }
            sqlx::Error::Io(_) => RepositoryError::Connection(err.to_string()),
            sqlx::Error::Tls(_) => RepositoryError::Connection(err.to_string()),
            _ => RepositoryError::Database(err.to_string()),
        }
    }
}
