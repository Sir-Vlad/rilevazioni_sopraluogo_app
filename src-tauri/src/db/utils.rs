use crate::app_traits::Insert;
use crate::dao::TipoInfissoDAO;
use crate::dao::{
    create_tables, ClimatizzazioneDAO, IlluminazioneDAO, MaterialeInfissoDAO, VetroInfissoDAO,
};
use crate::entities::{
    Climatizzazione, Illuminazione, MaterialeInfisso, TipoInfisso, VetroInfisso,
};
use dirs_next::document_dir;
use log::{error, info, warn};
use rusqlite::Transaction;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager};

pub const NAME_DIR_DATABASE: &str = "Dati_Sopralluogo";

pub fn create_or_get_db_path(db_name: String) -> Result<String, String> {
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
                Err(e) => return Err(format!("Errore nella creazione del db: {}", e)),
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
                for data in data {
                    MaterialeInfissoDAO::insert(
                        tx,
                        MaterialeInfisso::new(data.value, data.efficienza_energetica as u8),
                    )?;
                }
            }
            "vetro_infisso" => {
                for data in data {
                    VetroInfissoDAO::insert(
                        tx,
                        VetroInfisso::new(data.value, data.efficienza_energetica as u8),
                    )?;
                }
            }
            "climatizzazione" => {
                for data in data {
                    ClimatizzazioneDAO::insert(
                        tx,
                        Climatizzazione::new(data.value, data.efficienza_energetica as u8),
                    )?;
                }
            }
            "illuminazione" => {
                for data in data {
                    IlluminazioneDAO::insert(
                        tx,
                        Illuminazione::new(data.value, data.efficienza_energetica as u8),
                    )?;
                }
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
