use crate::database::utils::{get_db_path, init_database, NAME_DIR_DATABASE};
use dirs_next::document_dir;
use log::{info, warn};
use rusqlite::Connection;
use serde::Serialize;
use std::fs;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, State};

pub struct Database {
    pub(crate) conn: Mutex<Option<Connection>>,
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
    pub fn with_transaction<F, T>(&self, op: F) -> Result<T, String>
    where
        T: for<'de> serde::Deserialize<'de> + serde::Serialize,
        F: FnOnce(&rusqlite::Transaction) -> Result<T, String>,
    {
        let mut conn_guard = self.conn.lock().unwrap();
        if let Some(conn) = conn_guard.as_mut() {
            let tx = conn.transaction().map_err(|e| e.to_string())?;
            let result = op(&tx)?;
            tx.commit().map_err(|e| e.to_string())?;
            Ok(result)
        } else {
            Err("Database not initialized".to_string())
        }
    }
}

#[derive(Serialize, Clone)]
pub struct DatabaseEventPayload {
    pub(crate) type_event: &'static str,
    pub(crate) path: String,
}

#[tauri::command]
pub fn set_database(
    app_handle: AppHandle,
    db: State<'_, Database>,
    db_name: String,
) -> Result<String, String> {
    let db_path = get_db_path(db_name)?;
    let mut conn = db.conn.lock().unwrap();
    let mut path_to_database = db.path_to_database.lock().unwrap();
    if let Some(existing_conn) = conn.take() {
        drop(existing_conn);
    }
    *conn = Some(Connection::open(&db_path).map_err(|c| c.to_string())?);
    *path_to_database = Some(db_path.clone());

    setup_database(conn.as_ref().unwrap())?;
    match init_database(
        app_handle.clone(),
        conn.as_ref().ok_or("Database connection not initialized")?,
    ) {
        Ok(_) => info!("Database inizializzato"),
        Err(e) => {
            warn!("Errore nell'inizializzazione del database: {}", e);
            return Err(e.to_string());
        }
    };
    Ok(db_path)
}

#[tauri::command]
pub fn switch_database(
    app_handle: AppHandle,
    db: State<'_, Database>,
    db_name: String,
) -> Result<(), String> {
    let db_path = get_db_path(db_name)?;
    let mut conn = db.conn.lock().unwrap();
    let mut path_to_database = db.path_to_database.lock().unwrap();
    if let Some(existing_conn) = conn.take() {
        drop(existing_conn);
    }
    *conn = Some(Connection::open(&db_path).map_err(|c| c.to_string())?);
    *path_to_database = Some(db_path.clone());
    setup_database(conn.as_ref().unwrap())?;

    app_handle
        .emit(
            "database-changed",
            DatabaseEventPayload {
                type_event: "database_switched",
                path: db_path.clone(),
            },
        )
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_all_name_database() -> Result<Vec<String>, String> {
    if let Some(mut path) = document_dir() {
        path.push(NAME_DIR_DATABASE);
        if !path.exists() {
            return Err("Cartella non esiste".to_string());
        }
        // recupero tutti i nomi dei file all'interno della cartella
        let entries = fs::read_dir(path)
            .map_err(|e| e.to_string())?
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_file())
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .collect::<Vec<String>>();
        return Ok(entries);
    }
    Err("La cartella Documenti non è stata trovata".to_string())
}

fn setup_database(connection: &Connection) -> Result<(), String> {
    connection
        .pragma_update(None, "foreign_keys", "ON")
        .map_err(|e| format!("Errore durante l'impostazione delle pragma: {}", e))?;
    info!("Foreign keys enabled");
    Ok(())
}
