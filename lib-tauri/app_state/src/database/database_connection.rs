use crate::utils::AppError;
use rusqlite::Params;
use std::ops::Deref;

pub trait DatabaseConnection {
    fn prepare(&self, query: &str) -> Result<rusqlite::Statement, AppError>;
    fn execute<P: Params>(&self, query: &str, params: P) -> Result<usize, AppError>;
}

impl DatabaseConnection for rusqlite::Connection {
    fn prepare(&self, query: &str) -> Result<rusqlite::Statement, AppError> {
        Ok(rusqlite::Connection::prepare(self, query)?)
    }

    fn execute<P: Params>(&self, query: &str, params: P) -> Result<usize, AppError> {
        Ok(rusqlite::Connection::execute(self, query, params)?)
    }
}

impl DatabaseConnection for rusqlite::Transaction<'_> {
    fn prepare(&self, query: &str) -> Result<rusqlite::Statement, AppError> {
        Ok(rusqlite::Transaction::deref(self).prepare(query)?)
    }

    fn execute<P: Params>(&self, query: &str, params: P) -> Result<usize, AppError> {
        Ok(rusqlite::Transaction::deref(self).execute(query, params)?)
    }
}
