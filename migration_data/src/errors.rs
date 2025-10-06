use std::error::Error;
use thiserror::Error;

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
