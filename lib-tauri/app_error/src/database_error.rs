use diesel::result::Error as DieselError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Database error: {0}")]
    DieselError(#[from] DieselError),
    #[error("Connection pool error: {0}")]
    ConnectionPoolError(String),
    #[error("Connection timeout")]
    ConnectionTimeout,
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Generic error: {0}")]
    GenericError(String),
}

impl From<tokio::time::error::Elapsed> for DbError {
    fn from(_: tokio::time::error::Elapsed) -> Self { DbError::ConnectionTimeout }
}

// Per integrazione con Tauri
impl From<DbError> for String {
    fn from(error: DbError) -> Self { error.to_string() }
}

impl From<std::env::VarError> for DbError {
    fn from(error: std::env::VarError) -> Self {
        DbError::ConfigurationError(format!("Environment variable error: {error}"))
    }
}
