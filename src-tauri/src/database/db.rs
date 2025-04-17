use crate::database::utils::{get_db_path, init_database, NAME_DIR_DATABASE};
use crate::database::{Database, DatabaseEventPayload};
use dirs_next::document_dir;
use log::{info, warn};
use rusqlite::Connection;
use std::ffi::OsStr;
use std::fs;
use tauri::{AppHandle, Emitter, State};

#[tauri::command]
pub fn set_database(
    app_handle: AppHandle,
    db: State<'_, Database>,
    db_name: String,
) -> Result<String, String> {
    let db_path = get_db_path(db_name)?;
    let mut conn = db.get_conn();
    let mut path_to_database = db.get_path_to_database();
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
    let mut conn = db.get_conn();
    let mut path_to_database = db.get_path_to_database();
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
            .filter(|entry| {
                entry.path().is_file() && entry.path().extension() == Some(OsStr::new("db"))
            })
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
    connection
        .pragma_update(None, "journal_mode", "WAL")
        .map_err(|e| format!("Errore durante l'impostazione del pragma: {}", e))?;
    info!("Journal mode enabled");
    Ok(())
}
