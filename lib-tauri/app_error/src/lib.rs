use crate::database_error::DbError;
use std::fmt::{Display, Formatter};
use thiserror::Error;

pub mod database_error;

pub type AppResult<T> = Result<T, ApplicationError>;

macro_rules! define_domain_errors {
    (
        entities: [$(($entity:ident, $display_name:literal)),* $(,)?],
        singles: [$(($single:ident, $single_display:literal)),* $(,)?],
        custom: [$($variant:ident $( ( $($field:ty),* ) )? => $msg:literal),* $(,)?]
    ) => {
        paste::paste! {
        #[derive(Error, Debug, PartialEq)]
        pub enum DomainError {
            $(
                #[error("{} not found", $display_name)]
                [<$entity NotFound>],
                #[error("{} already exists", $display_name)]
                [<$entity AlreadyExists>],
            )*

            $(
                #[error("{} not found", $single_display)]
                [<$single NotFound>],
            )*

            $(
                #[error($msg)]
                $variant $( ( $($field), * ), )?
            )*
        }
        }
    };
}



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

impl From<DbError> for ApplicationError {
    fn from(value: DbError) -> Self {
        ApplicationError::Infrastructure(InfrastructureError::DatabaseError(value))
    }
}

define_domain_errors! {
    entities: [
        (Edificio, "Edificio"),
        (Infisso, "Infisso"),
        (Stanza, "Stanza"),
        (Annotazione, "Annotazione"),
        (Fotovoltaico, "Fotovoltaico"),
        (Illuminazione, "Illuminazione"),
        (Climatizzazione, "Climatizzazione"),
        (MaterialeInfisso, "Materiale Infisso"),
        (Utenza, "Utenza"),
    ],
    singles: [
        (StanzaConInfissi, "Stanza con infissi"),
    ],
    custom: [
        TipoInvalid(String) => "Tipo invalid: {0}",
        InvalidInput(ErrorKind, String) => "Invalid input: {0}",
        Unexpected(diesel::result::Error) => "Unexpected error: {0}",
    ]
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
                    DomainError::Unexpected(value)
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
    FormatInvalid,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::EmptyField => write!(f, "Empty field"),
            ErrorKind::FormatInvalid => write!(f, "Format invalid"),
        }
    }
}

#[derive(Error, Debug)]
pub enum InfrastructureError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] DbError),
    #[error("Connection pool error: {0}")]
    ConnectionPool(String),
    #[error("Connection timeout")]
    ConnectionTimeout,
}
