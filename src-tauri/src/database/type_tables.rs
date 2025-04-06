use crate::database::Database;
use rusqlite::Connection;
use std::collections::HashMap;
use tauri::State;

#[tauri::command]
pub fn get_types(db: State<'_, Database>) -> Result<HashMap<&'static str, Vec<String>>, String> {
    let mut types: HashMap<&str, Vec<String>> = HashMap::new();
    let conn = db.get_conn();
    if let Some(conn) = conn.as_ref() {
        types.insert("materiale_infissi", get_materiale_infissi_type(conn));
        types.insert("vetro_infissi", get_vetro_infissi_type(conn));
        types.insert("climatizzazione", get_climatizzazione_type(conn));
        types.insert("illuminazione", get_illuminazione_type(conn));
        Ok(types)
    } else {
        Err("Database not initialized".to_string())
    }
}

fn get_materiale_infissi_type(connection: &Connection) -> Vec<String> {
    let mut stmt = connection
        .prepare("SELECT MATERIALE FROM MATERIALE_INFISSO")
        .unwrap();

    let result: Result<Vec<String>, rusqlite::Error> = stmt
        .query_map([], |row| row.get(0))
        .expect("Errore nella lettura dei dati di tipo materiale")
        .collect();
    match result {
        Ok(types) => types,
        Err(e) => panic!("Errore nella lettura dei dati di tipo materiale: {}", e),
    }
}

fn get_vetro_infissi_type(connection: &Connection) -> Vec<String> {
    let mut stmt = connection
        .prepare("SELECT VETRO FROM VETRO_INFISSO")
        .unwrap();

    let result: Result<Vec<String>, rusqlite::Error> = stmt
        .query_map([], |row| row.get(0))
        .expect("Errore nella lettura dei dati di tipo materiale")
        .collect();
    match result {
        Ok(types) => types,
        Err(e) => panic!("Errore nella lettura dei dati di tipo materiale: {}", e),
    }
}

fn get_climatizzazione_type(connection: &Connection) -> Vec<String> {
    let mut stmt = connection
        .prepare("SELECT CLIMATIZZAZIONE FROM CLIMATIZZAZIONE")
        .unwrap();

    let result: Result<Vec<String>, rusqlite::Error> = stmt
        .query_map([], |row| row.get(0))
        .expect("Errore nella lettura dei dati di tipo materiale")
        .collect();
    match result {
        Ok(types) => types,
        Err(e) => panic!("Errore nella lettura dei dati di tipo materiale: {}", e),
    }
}
fn get_illuminazione_type(connection: &Connection) -> Vec<String> {
    let mut stmt = connection
        .prepare("SELECT LAMPADINA FROM ILLUMINAZIONE")
        .unwrap();

    let result: Result<Vec<String>, rusqlite::Error> = stmt
        .query_map([], |row| row.get(0))
        .expect("Errore nella lettura dei dati di tipo materiale")
        .collect();
    match result {
        Ok(types) => types,
        Err(e) => panic!("Errore nella lettura dei dati di tipo materiale: {}", e),
    }
}
