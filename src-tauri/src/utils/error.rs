use rusqlite::Error;
use tauri::ipc::InvokeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database not initialized")]
    DatabaseNotInitialized,
    #[error("Database error: {0}")]
    DatabaseError(rusqlite::Error),
    #[error("Query build error: {0}")]
    QueryBuildError(#[from] crate::database::QueryBuilderError),
    #[error("Not found element: {0}")]
    NotFound(String),
    #[error("")]
    TauriError(#[from] tauri::Error),
    #[error("")]
    PolarsError(#[from] polars::error::PolarsError),
    #[error("")]
    CalamineError(#[from] calamine::Error),
    #[error("")]
    GenericError(String),
    #[error("")]
    IdInvalid(#[from] crate::service::import::Error),
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

impl From<InvokeError> for AppError {
    fn from(value: InvokeError) -> Self {
        AppError::GenericError(format!("{:?}", value))
    }
}
