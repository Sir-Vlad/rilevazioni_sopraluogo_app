use thiserror::Error;

pub mod database_error;

pub type AppResult<T> = Result<T, ApplicationError>;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),
    #[error("Infrastructure error: {0}")]
    Infrastructure(#[from] InfrastructureError),
    #[error("Unknown error")]
    Unknown,
    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Edificio not found: {0}")]
    EdificioNotFound(String),
    #[error("Infisso not found: {0}")]
    InfissoNotFound(String),
    #[error("Stanza not found: {0}")]
    StanzaNotFound(String),
    #[error("Infisso already exists: {0}")]
    InfissoAlreadyExists(String),
    #[error("Stanza already exists: {0}")]
    StanzaAlreadyExists(String),
    #[error("Edificio already exists: {0}")]
    EdificioAlreadyExists(String),
    #[error("Unexpected error: {0}")]
    Unexpected(#[from] diesel::result::Error),
}

#[derive(Error, Debug)]
pub enum InfrastructureError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] crate::database_error::DbError),
    #[error("Connection pool error: {0}")]
    ConnectionPool(String),
    #[error("Connection timeout")]
    ConnectionTimeout,
}
