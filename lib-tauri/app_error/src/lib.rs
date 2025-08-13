use crate::database_error::DbError;
use std::fmt::{Display, Formatter};
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
    #[error("Edificio not selected")]
    EdificioNotSelected,
}

impl From<database_error::DbError> for ApplicationError {
    fn from(value: DbError) -> Self {
        ApplicationError::Infrastructure(InfrastructureError::DatabaseError(value))
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum DomainError {
    #[error("Edificio not found")]
    EdificioNotFound,
    #[error("Edificio already exists")]
    EdificioAlreadyExists,

    #[error("Infisso not found")]
    InfissoNotFound,
    #[error("Infisso already exists")]
    InfissoAlreadyExists,

    #[error("Stanza not found")]
    StanzaNotFound,
    #[error("Stanza already exists")]
    StanzaAlreadyExists,

    #[error("Annotazione not found")]
    AnnotazioneNotFound,
    #[error("Annotazione already exists")]
    AnnotazioneAlreadyExists,

    #[error("Fotovoltaico not found")]
    FotovoltaicoNotFound,
    #[error("Fotovoltaico already exists")]
    FotovoltaicoAlreadyExists,

    #[error("Illuminazione not found")]
    IlluminazioneNotFound,
    #[error("Illuminazione already exists")]
    IlluminazioneAlreadyExists,

    #[error("Materiale Infisso not found")]
    MaterialeInfissoNotFound,
    #[error("Materiale Infisso already exists")]
    MaterialeInfissoAlreadyExists,

    #[error("Stanza con infissi not found")]
    StanzaConInfissiNotFound,

    #[error("Utenza not found")]
    UtenzaNotFound,
    #[error("Utenza already exists")]
    UtenzaAlreadyExists,

    #[error("Invalid input: {0}")]
    InvalidInput(ErrorKind, String),

    #[error("Unexpected error: {0}")]
    Unexpected(diesel::result::Error),
}

impl From<diesel::result::Error> for DomainError {
    fn from(value: diesel::result::Error) -> Self {
        match value {
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::Unknown,
                ref msg,
            ) => {
                if msg.message() == "Field content cannot be empty or contain only whitespace" {
                    DomainError::InvalidInput(ErrorKind::EmptyField, msg.message().to_string())
                } else {
                    DomainError::from(value)
                }
            }
            _ => DomainError::Unexpected(value),
        }
    }
}

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum ErrorKind {
    EmptyField,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::EmptyField => write!(f, "Empty field"),
        }
    }
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
