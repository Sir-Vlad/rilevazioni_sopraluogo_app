use crate::utils::AppError;
use log::info;
use rusqlite::{Connection, ToSql};
use serde::Serialize;
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
    pub fn get_path_to_database(&self) -> MutexGuard<'_, Option<String>> {
        self.path_to_database.lock().unwrap()
    }

    pub fn get_conn(&self) -> MutexGuard<'_, Option<Connection>> {
        self.conn.lock().unwrap()
    }

    pub fn with_transaction<F, T>(&self, op: F) -> Result<T, AppError>
    where
        T: for<'de> serde::Deserialize<'de> + serde::Serialize,
        F: FnOnce(&rusqlite::Transaction) -> Result<T, AppError>,
    {
        let mut conn_guard = self.get_conn();
        if let Some(conn) = conn_guard.as_mut() {
            let tx = conn.transaction()?;
            let result = op(&tx)?;
            tx.commit()?;
            Ok(result)
        } else {
            Err(AppError::DatabaseNotInitialized)
        }
    }

    pub fn open_in_memory() -> Self {
        Self {
            conn: Mutex::new(Connection::open_in_memory().ok()),
            path_to_database: Mutex::new(Some(":memory:".to_string())),
        }
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if let Ok(mut conn_guard) = self.conn.lock() {
            if let Some(conn) = conn_guard.take() {
                if let Err((_, e)) = conn.close() {
                    eprintln!(
                        "Errore durante la chiusura del database nel destructor: {}",
                        e
                    );
                }
                info!("Database chiuso");
            }
        }
    }
}

#[derive(Serialize, Clone)]
pub struct DatabaseEventPayload {
    pub(crate) type_event: &'static str,
    pub(crate) path: String,
}
