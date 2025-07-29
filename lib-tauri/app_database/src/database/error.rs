use diesel::result::Error as DieselError;
use std::error::Error;
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
    #[error("Migrations error: {0:?}")]
    MigrationError(#[from] MigrationError),
    #[error("Generic error: {0}")]
    GenericError(String),
}

impl From<tokio::time::error::Elapsed> for DbError {
    fn from(_: tokio::time::error::Elapsed) -> Self {
        DbError::ConnectionTimeout
    }
}

// Per integrazione con Tauri
impl From<DbError> for String {
    fn from(error: DbError) -> Self {
        error.to_string()
    }
}

impl From<std::env::VarError> for DbError {
    fn from(error: std::env::VarError) -> Self {
        DbError::ConfigurationError(format!("Environment variable error: {error}"))
    }
}

#[derive(Error, Debug)]
pub enum MigrationError {
    #[error("Connection error of database: {0}")]
    Connection(#[from] diesel::result::Error),
    #[error("Error during migration execution: {0}")]
    SchemaMigration(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error("Error during data migration: {0}")]
    DataMigration(#[from] DataMigrationError),
}

#[derive(Error, Debug)]
pub enum DataMigrationError {
    #[error("The migration isn't supported yet")]
    UnsupportedMigration,
    #[error("Error diesel: {0}")]
    DieselError(#[from] diesel::result::Error),
    #[error("Error: {0}")]
    GenericError(String),
}

impl From<Box<dyn Error>> for DataMigrationError {
    fn from(value: Box<dyn Error>) -> Self {
        DataMigrationError::GenericError(value.to_string())
    }
}
