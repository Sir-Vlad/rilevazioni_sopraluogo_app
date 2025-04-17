use rusqlite::Params;
use std::ops::Deref;

pub trait DatabaseConnection {
    fn prepare(&self, query: &str) -> Result<rusqlite::Statement, String>;
    fn execute<P: Params>(&self, query: &str, params: P) -> Result<usize, String>;
}

impl DatabaseConnection for rusqlite::Connection {
    fn prepare(&self, query: &str) -> Result<rusqlite::Statement, String> {
        rusqlite::Connection::prepare(self, query).map_err(|e| e.to_string())
    }

    fn execute<P: Params>(&self, query: &str, params: P) -> Result<usize, String> {
        rusqlite::Connection::execute(self, query, params).map_err(|e| e.to_string())
    }
}

impl DatabaseConnection for rusqlite::Transaction<'_> {
    fn prepare(&self, query: &str) -> Result<rusqlite::Statement, String> {
        rusqlite::Transaction::deref(self)
            .prepare(query)
            .map_err(|e| e.to_string())
    }

    fn execute<P: Params>(&self, query: &str, params: P) -> Result<usize, String> {
        rusqlite::Transaction::deref(self)
            .execute(query, params)
            .map_err(|e| e.to_string())
    }
}
