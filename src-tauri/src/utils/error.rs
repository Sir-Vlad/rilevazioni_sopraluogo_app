use rusqlite::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(rusqlite::Error),
    #[error("Query build error: {0}")]
    QueryBuildError(#[from] crate::database::QueryBuilderError),
    #[error("Not found element: {0}")]
    NotFound(String),
}

impl From<rusqlite::Error> for AppError {
    fn from(value: Error) -> Self {
        AppError::DatabaseError(value)
    }
}

impl From<AppError> for String {
    fn from(value: AppError) -> Self {
        value.to_string()
    }
}
