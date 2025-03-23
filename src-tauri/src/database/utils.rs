use dirs_next::document_dir;
use log::{info, warn};
use rusqlite::Connection;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::Path;

pub(crate) const NAME_DIR_DATABASE: &str = "Dati_Sopralluogo";

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

pub fn init_database(conn: &Connection) -> Result<(), rusqlite::Error> {
    create_infissi_table(conn)?;
    create_commenti_infisso_table(conn)?;
    create_stanze_table(conn)?;
    create_commenti_stanza_table(conn)?;
    create_stanze_con_infissi(conn)?;

    create_tipo_materiale_infisso(conn)?;
    create_tipo_vetro_infisso(conn)?;
    create_tipo_climatizzazione(conn)?;
    create_tipo_illuminazione(conn)?;

    let type_data = match retrieve_type_to_file("type.json") {
        Ok(data) => data,
        Err(e) => {
            return Err(rusqlite::Error::InvalidParameterName(format!(
                "JsonParseError: {}",
                e
            )))
        }
    };
    for (table_name, data) in type_data {
        let name_table = format!("TIPO_{}", table_name);
        match table_name.as_str() {
            "materiale_infisso" => {
                insert_values_into_table(conn, name_table.as_str(), "MATERIALE", data)
            }
            "vetro_infisso" => insert_values_into_table(conn, name_table.as_str(), "VETRO", data),
            "climatizzazione" => {
                insert_values_into_table(conn, name_table.as_str(), "CLIMATIZZAZIONE", data)
            }
            "illuminazione" => {
                insert_values_into_table(conn, name_table.as_str(), "LAMPADINA", data)
            }
            _ => warn!("Tabella {} non presente", table_name),
        }
    }
    Ok(())
}

fn create_infissi_table(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS INFISSI (
                ID        TEXT PRIMARY KEY,
                TIPO      TEXT DEFAULT 'Finestra' CHECK ( TIPO IN ('Finestra', 'Porta')),
                ALTEZZA   INTEGER NOT NULL,
                LARGHEZZA INTEGER NOT NULL,
                MATERIALE TEXT NOT NULL,
                VETRO     TEXT NOT NULL
            );",
        [],
    )
    .map(|_| info!("Tabella INFISSI creata con successo"))
}

fn create_stanze_table(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS STANZE (
            ID               INTEGER PRIMARY KEY AUTOINCREMENT,
            FASCICOLO        TEXT NOT NULL,
            PIANO            TEXT NOT NULL,
            ID_SPAZIO        TEXT NOT NULL,
            STANZA           TEXT NOT NULL,
            DESTINAZIONE_USO TEXT NOT NULL,
            ALTEZZA          INTEGER,
            SPESSORE_MURO    INTEGER,
            RISCALDAMENTO    TEXT,
            RAFFRESCAMENTO   TEXT,
            ILLUMINAZIONE    TEXT,
            UNIQUE (ID_SPAZIO, STANZA, DESTINAZIONE_USO)
        );",
        [],
    )
    .map(|_| info!("Tabella STANZE creata con successo"))
}

fn create_stanze_con_infissi(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS STANZE_CON_INFISSI (
            ID_STANZA  INTEGER NOT NULL,
            ID_INFISSI TEXT NOT NULL,
            RIPETIZIONE INTEGER NOT NULL DEFAULT 1,
            PRIMARY KEY (ID_INFISSI, ID_STANZA),
            FOREIGN KEY (ID_INFISSI) REFERENCES INFISSI (ID),
            FOREIGN KEY (ID_STANZA) REFERENCES STANZE (ID)
        )",
        [],
    )
    .map(|_| info!("Tabella STANZE_CON_INFISSI creata con successo"))
}

fn create_tipo_materiale_infisso(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS TIPO_MATERIALE_INFISSO (
            ID        INTEGER PRIMARY KEY AUTOINCREMENT,
            MATERIALE TEXT NOT NULL UNIQUE
        )",
        [],
    )
    .map(|_| info!("Tabella TIPO_MATERIALE_INFISSO creata con successo"))
}

fn create_tipo_vetro_infisso(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS TIPO_VETRO_INFISSO (
            ID    INTEGER PRIMARY KEY AUTOINCREMENT,
            VETRO TEXT NOT NULL UNIQUE
        )",
        [],
    )
    .map(|_| info!("Tabella TIPO_VETRO_INFISSO creata con successo"))
}

fn create_tipo_climatizzazione(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS TIPO_CLIMATIZZAZIONE (
            ID              INTEGER PRIMARY KEY AUTOINCREMENT,
            CLIMATIZZAZIONE TEXT NOT NULL UNIQUE
        )",
        [],
    )
    .map(|_| info!("Tabella TIPO_CLIMATIZZAZIONE creata con successo"))
}

fn create_tipo_illuminazione(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS TIPO_ILLUMINAZIONE (
            ID        INTEGER PRIMARY KEY AUTOINCREMENT,
            LAMPADINA TEXT NOT NULL UNIQUE
        )",
        [],
    )
    .map(|_| info!("Tabella TIPO_ILLUMINAZIONE creata con successo"))
}

fn create_commenti_stanza_table(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS COMMENTI_STANZA (
            ID_STANZA INTEGER PRIMARY KEY,
            CONTENT  TEXT NOT NULL,
            INSERT_DATE TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (ID_STANZA) REFERENCES STANZE (ID)
        )",
        [],
    )
    .map(|_| info!("Tabella COMMENTI_STANZA creata con successo"))
}

fn create_commenti_infisso_table(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS COMMENTI_INFISSO (
            ID_INFISSO TEXT PRIMARY KEY,
            CONTENT  TEXT NOT NULL,
            INSERT_DATE TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (ID_INFISSO) REFERENCES INFISSI (ID)
        )",
        [],
    )
    .map(|_| info!("Tabella COMMENTI_STANZA creata con successo"))
}

type JsonTypeMap = HashMap<String, Vec<String>>;
fn retrieve_type_to_file(file_name: &str) -> Result<JsonTypeMap, String> {
    let path = Path::new("src/database/type").join(file_name);
    let file_content = fs::read_to_string(&path)
        .map_err(|e| format!("Errore nella lettura del file {}: {}", file_name, e))?;
    let data: JsonTypeMap = serde_json::from_str(&file_content)
        .map_err(|e| format!("Errore nella deserializzazione di {}: {}", file_name, e))?;
    Ok(data)
}

fn insert_values_into_table(
    conn: &Connection,
    table_name: &str,
    column_name: &str,
    values: Vec<String>,
) {
    let query = format!(
        "INSERT OR IGNORE INTO {}({}) VALUES (?1)",
        table_name, column_name
    );
    let mut stmt = conn
        .prepare(&query)
        .expect("Errore nella preparazione della query per inserire i dati nel database");
    for value in values {
        stmt.execute(&[&value])
            .expect("Errore nell'inserimento dei dati nel database");
    }
    info!("Tabella {} popolata con successo", table_name);
}
