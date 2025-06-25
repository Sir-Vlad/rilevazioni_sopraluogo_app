use crate::app_traits::SqlExecutor;
use crate::utils::AppError;
use log::info;
use rusqlite::{Connection, Statement, ToSql, Transaction};
use serde::Serialize;
use std::ops::Deref;
use std::sync::{Mutex, MutexGuard};

pub struct Database {
    conn: Mutex<Option<Connection>>,
    path_to_database: Mutex<Option<String>>,
}

impl Default for Database {
    fn default() -> Self {
        Self {
            conn: Mutex::new(None),
            path_to_database: Mutex::new(None),
        }
    }
}

impl Database {
    pub fn new(url: String) -> Result<Self, AppError> {
        let connection = Connection::open(&url).map_err(AppError::DatabaseError)?;
        let instance = Self {
            conn: Mutex::new(Some(connection)),
            path_to_database: Mutex::new(Some(url)),
        };
        instance.set_pragmas()?;
        Ok(instance)
    }

    #[cfg(test)] // serve solo per i test
    pub fn open_in_memory() -> Self {
        let instance = Self {
            conn: Mutex::new(Connection::open_in_memory().ok()),
            path_to_database: Mutex::new(Some(":memory:".to_string())),
        };
        instance.set_pragmas().unwrap();
        instance
    }

    pub fn get_path_to_database(&self) -> Result<MutexGuard<'_, Option<String>>, AppError> {
        self.path_to_database
            .lock()
            .map_err(|_| AppError::GenericError("Failed to acquire connection lock".to_string()))
    }

    pub fn get_conn(&self) -> Result<MutexGuard<'_, Option<Connection>>, AppError> {
        self.conn
            .lock()
            .map_err(|_| AppError::GenericError("Failed to acquire connection lock".to_string()))
    }

    pub fn with_transaction<F, T>(&self, op: F) -> Result<T, AppError>
    where
        T: for<'de> serde::Deserialize<'de> + serde::Serialize,
        F: FnOnce(&rusqlite::Transaction) -> Result<T, AppError>,
    {
        let mut conn_guard = self.get_conn()?;
        if let Some(conn) = conn_guard.as_mut() {
            let tx = conn.transaction()?;
            let result = op(&tx);
            match result {
                Ok(result) => {
                    tx.commit()?;
                    Ok(result)
                }
                Err(err) => {
                    tx.rollback()?;
                    Err(AppError::GenericError(format!("Rollback: {}", err)))
                }
            }
        } else {
            Err(AppError::DatabaseNotInitialized)
        }
    }

    pub fn switch_database(&self, url: &str) -> Result<(), AppError> {
        let new_connection = Connection::open(url)?;
        {
            let mut conn_guard = self.get_conn()?;
            let mut path_guard = self.get_path_to_database()?;

            if let Some(conn) = conn_guard.take() {
                if let Err((returned_conn, e)) = conn.close() {
                    // If closing the connection fails, the error is returned and
                    // restore the previous connection
                    *conn_guard = Some(returned_conn);
                    return Err(AppError::DatabaseError(e));
                }
            }

            *conn_guard = Some(new_connection);
            *path_guard = Some(url.to_string());
        } // unlock conn_guard
        self.set_pragmas()?;
        Ok(())
    }

    pub fn close(&self) -> Result<(), AppError> {
        let mut conn_guard = self.get_conn()?;
        let mut path_guard = self.get_path_to_database()?;

        if let Some(conn) = conn_guard.take() {
            if let Err((_, e)) = conn.close() {
                eprintln!("Errore durante la chiusura del db nel destructor: {}", e);
            } else {
                *path_guard = None;
            }
            info!("Database chiuso");
        }
        Ok(())
    }

    fn set_pragmas(&self) -> Result<(), AppError> {
        if let Some(conn) = self.get_conn()?.as_ref() {
            conn.pragma_update(None, "foreign_keys", "ON")?;
            conn.pragma_update(None, "journal_mode", "WAL")?;
            Ok(())
        } else {
            Err(AppError::GenericError("Pragma update failed".to_string()))
        }
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        Self::close(self);
    }
}

#[derive(Serialize, Clone)]
pub struct DatabaseEventPayload {
    pub(crate) type_event: &'static str,
    pub(crate) path: String,
}

impl SqlExecutor for Connection {
    fn execute(&self, sql: &str, params: &[&dyn ToSql]) -> rusqlite::Result<usize> {
        self.execute(sql, params)
    }

    fn prepare(&self, sql: &str) -> rusqlite::Result<Statement> {
        self.prepare(sql)
    }
}

impl SqlExecutor for Transaction<'_> {
    fn execute(&self, query: &str, params: &[&dyn ToSql]) -> rusqlite::Result<usize> {
        rusqlite::Transaction::deref(self).execute(query, params)
    }

    fn prepare(&self, query: &str) -> rusqlite::Result<rusqlite::Statement> {
        rusqlite::Transaction::deref(self).prepare(query)
    }
}
