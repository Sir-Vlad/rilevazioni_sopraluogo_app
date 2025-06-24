use crate::app_traits::Insert;
use crate::dao::create_tables;
use crate::dao::crud_operations::Insert as OldInsert;
use crate::dao::{entity::TipoInfisso, TipoInfissoDAO};
use crate::database::{DatabaseConnection, QueryParam};
use dirs_next::document_dir;
use log::{error, info, warn};
use rusqlite::{params, Connection, Transaction};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager};

pub const NAME_DIR_DATABASE: &str = "Dati_Sopralluogo";

pub fn get_db_path(db_name: String) -> Result<String, String> {
    if let Some(mut path) = document_dir() {
        // creo la cartella per salvare i db
        path.push(NAME_DIR_DATABASE);
        if !path.exists() {
            match fs::create_dir_all(&path) {
                Ok(()) => println!("Cartella creata correttamente"),
                Err(e) => return Err(format!("Errore nella creazione della cartella: {}", e)),
            }
        }
        // creo il file db, se necessario, e ritorno il suo path
        path.push(format!("{}.db", db_name));
        if !path.exists() {
            match File::create(&path) {
                Ok(_) => println!("Database creato correttamente"),
                Err(e) => return Err(format!("Errore nella creazione del database: {}", e)),
            }
        }
        return Ok(path.to_string_lossy().to_string());
    }
    Err("Impossibile determinare la directory dei documenti".to_string())
}

pub fn init_database(app_handle: AppHandle, tx: &Transaction) -> Result<(), String> {
    create_tables(tx).map_err(|e| {
        error!("{}", e);
        e.to_string()
    })?;

    let type_data = retrieve_type_to_file(app_handle, "type.json")?;
    for (table_name, data) in type_data {
        match table_name.as_str() {
            "materiale_infisso" => {
                insert_values_into_table(tx, table_name.as_str(), "MATERIALE", data)?
            }
            "vetro_infisso" => insert_values_into_table(tx, table_name.as_str(), "VETRO", data)?,
            "climatizzazione" => {
                insert_values_into_table(tx, table_name.as_str(), "CLIMATIZZAZIONE", data)?
            }
            "illuminazione" => {
                insert_values_into_table(tx, table_name.as_str(), "LAMPADINA", data)?
            }
            _ => warn!("Tabella {} non presente", table_name),
        }
    }

    for tipo in vec![
        "FINESTRA",
        "PORTA",
        "VETRATA",
        "PORTA-FINESTRA",
        "LUCERNARIO",
    ]
    .into_iter()
    {
        let tipo_infisso = TipoInfisso {
            _id: 0,
            nome: tipo.to_string(),
        };
        TipoInfissoDAO::insert(tx, tipo_infisso)?;
    }
    info!("Tabella TIPO_INFISSO popolata con successo");

    Ok(())
}

pub fn set_pragma(connection: &Connection) -> Result<(), rusqlite::Error> {
    connection.pragma_update(None, "foreign_keys", "ON")?;
    info!("Foreign keys enabled");
    connection.pragma_update(None, "journal_mode", "WAL")?;
    info!("Journal mode enabled");
    Ok(())
}

#[derive(Deserialize)]
struct TypeRecord {
    value: String,
    efficienza_energetica: i32,
}
type JsonTypeMap = HashMap<String, Vec<TypeRecord>>;
fn retrieve_type_to_file(app_handle: AppHandle, file_name: &str) -> Result<JsonTypeMap, String> {
    let path = app_handle
        .path()
        .resolve(format!("resources/{}", file_name), BaseDirectory::Resource)
        .map_err(|e| format!("Errore: {}", e))?;
    let file_content = fs::read_to_string(&path)
        .map_err(|e| format!("Errore nella lettura del file {}: {}", file_name, e))?;
    let data: JsonTypeMap = serde_json::from_str(&file_content)
        .map_err(|e| format!("Errore nella deserializzazione di {}: {}", file_name, e))?;
    Ok(data)
}

fn insert_values_into_table<C: DatabaseConnection>(
    conn: &C,
    table_name: &str,
    column_name: &str,
    values: Vec<TypeRecord>,
) -> Result<(), String> {
    let query = format!(
        "INSERT OR IGNORE INTO {}({}, EFFICIENZA_ENERGETICA) VALUES (?1, ?2)",
        table_name, column_name
    );
    let mut stmt = conn
        .prepare(&query)
        .map_err(|_e| "Errore nella preparazione della query per inserire i dati nel database")?;
    for value in values {
        stmt.execute(params![
            value.value.to_ascii_uppercase(),
            value.efficienza_energetica
        ])
        .map_err(|_e| "Errore nell'inserimento dei dati nel database")?;
    }
    info!("Tabella {} popolata con successo", table_name);
    Ok(())
}

pub fn convert_param(params: Vec<&QueryParam>) -> Vec<rusqlite::types::Value> {
    params
        .iter()
        .map(|p| match p {
            QueryParam::String(s) => rusqlite::types::Value::Text(s.clone()),
            QueryParam::Integer(i) => rusqlite::types::Value::Integer(*i),
            QueryParam::Float(f) => rusqlite::types::Value::Real(*f),
            QueryParam::Boolean(b) => rusqlite::types::Value::Integer(if *b { 1 } else { 0 }),
            QueryParam::Null => rusqlite::types::Value::Null,
        })
        .collect()
}
