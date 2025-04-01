use crate::database::model::{Stanza, StanzaBuilder};
use crate::database::{set_database, Database, DatabaseEventPayload};
use calamine::{open_workbook, Reader, Xlsx};
use polars::frame::{DataFrame, UniqueKeepStrategy};
use polars::prelude::{NamedFrom, Series};
use rusqlite::params;
use tauri::{AppHandle, Emitter, State};

#[tauri::command]
pub fn get_stanze(db: State<'_, Database>) -> Result<Vec<Stanza>, String> {
    let conn = db.conn.lock().unwrap();
    if let Some(conn) = conn.as_ref() {
        let mut stmt = conn.prepare("SELECT * FROM STANZA").ok().unwrap();
        let result: Result<Vec<Stanza>, rusqlite::Error> = stmt
            .query_map([], |row| {
                let stanza = StanzaBuilder::new()
                    .id(row.get::<_, u64>(0)?)
                    .fascicolo(row.get::<_, String>(1)?)
                    .piano(row.get::<_, String>(2)?)
                    .id_spazio(row.get::<_, String>(3)?)
                    .stanza(row.get::<_, String>(4)?)
                    .destinazione_uso(row.get::<_, String>(5)?)
                    .altezza(row.get::<_, Option<u16>>(6)?)
                    .spessore_muro(row.get::<_, Option<u8>>(7)?)
                    .riscaldamento(row.get::<_, Option<String>>(8)?)
                    .raffrescamento(row.get::<_, Option<String>>(9)?)
                    .illuminazione(row.get::<_, Option<String>>(10)?)
                    .build();
                Ok(stanza.ok().unwrap())
            })
            .map_err(|e| {
                format!(
                    "Errore nella lettura dei dati dal database: {:?}",
                    e.to_string()
                )
            })?
            .collect();

        match result {
            Ok(stanze) => Ok(stanze),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database not initialized".to_string())
    }
}
#[tauri::command]
pub fn insert_stanze(
    app_handle: AppHandle,
    db: State<'_, Database>,
    path: String,
) -> Result<String, String> {
    let df = elaborate_file(path)?;

    let name_db = df
        .column("fascicolo")
        .unwrap()
        .get(0)
        .unwrap()
        .to_string()
        .replace("\"", "");
    let path_db = set_database(app_handle.clone(), db.clone(), name_db)?;

    let conn = db.conn.lock().unwrap();
    if let Some(conn) = conn.as_ref() {
        conn.execute("BEGIN TRANSACTION", [])
            .map_err(|e| format!("Errore nella transazione: {}", e))?;

        let mut stmt = conn
            .prepare(
                "INSERT INTO EDIFICIO(CHIAVE, FASCICOLO, INDIRIZZO)
                VALUES (?1, ?2, ?3)",
            )
            .map_err(|e| format!("Errore nella preparazione della query: {}", e))?;

        let chiave = retrieve_string_field_df(&df, "chiave", 0)?;
        let fascicolo = retrieve_string_field_df(&df, "fascicolo", 0)?;
        let indirizzo_edificio = retrieve_string_field_df(&df, "nome_via", 0)?;

        stmt.execute(params![chiave, fascicolo, indirizzo_edificio])
            .map_err(|e| format!("Errore nella esecuzione della query: {}", e))?;

        stmt = conn
            .prepare(
                "INSERT INTO STANZA(CHIAVE, PIANO, ID_SPAZIO, STANZA, DESTINAZIONE_USO)
                    VALUES (?1, ?2, ?3, ?4, ?5)",
            )
            .map_err(|e| format!("Errore nella preparazione della query: {}", e))?;

        for i in 0..df.height() {
            let piano = retrieve_string_field_df(&df, "piano", i)?;
            let id_spazio = retrieve_string_field_df(&df, "id_spazio", i)?;
            let stanza = retrieve_string_field_df(&df, "cod_stanza", i)?;
            let destinazione_uso = retrieve_string_field_df(&df, "destinazione_uso", i)?;

            stmt.execute(params![chiave, piano, id_spazio, stanza, destinazione_uso])
                .map_err(|e| format!("Errore nella esecuzione della query: {}", e))?;
        }
        conn.execute("COMMIT TRANSACTION", [])
            .map_err(|e| format!("Errore nella transazione: {}", e))?;

        // emit event del cambio di database
        app_handle
            .emit(
                "database-changed",
                DatabaseEventPayload {
                    type_event: "database_switched",
                    path: path_db.clone(),
                },
            )
            .map_err(|e| e.to_string())?;

        Ok(path_db)
    } else {
        Err("Database not initialized".to_string())
    }
}

#[tauri::command]
pub fn update_stanza(db: State<'_, Database>, updated_stanza: Stanza) -> Result<(), String> {
    let conn = db.conn.lock().unwrap();
    if let Some(conn) = conn.as_ref() {
        let row_affected = conn
            .execute(
                "UPDATE STANZE
                SET ALTEZZA        = ?1,
                    SPESSORE_MURO  = ?2,
                    RISCALDAMENTO  = ?3,
                    RAFFRESCAMENTO = ?4,
                    ILLUMINAZIONE  = ?5
                WHERE ID = ?6
                  AND (ALTEZZA IS NULL OR SPESSORE_MURO IS NULL OR RISCALDAMENTO IS NULL OR
                       RAFFRESCAMENTO IS NULL OR ILLUMINAZIONE IS NULL)",
                params![
                    updated_stanza.altezza,
                    updated_stanza.spessore_muro,
                    updated_stanza.riscaldamento,
                    updated_stanza.raffrescamento,
                    updated_stanza.illuminazione,
                    updated_stanza.id
                ],
            )
            .map_err(|e| format!("Errore nella esecuzione della query: {}", e))?;

        if row_affected == 0 {
            return Err("Nessun record aggiornato".to_string());
        }
        Ok(())
    } else {
        Err("Database not initialized".to_string())
    }
}

fn retrieve_string_field_df(df: &DataFrame, field: &str, index: usize) -> Result<String, String> {
    Ok(df
        .column(field)
        .map_err(|e| format!("Errore nella lettura della colonna {field}: {}", e))?
        .str()
        .map_err(|e| format!("Errore nella lettura della colonna {field}: {}", e))?
        .get(index)
        .unwrap()
        .to_string())
}

fn elaborate_file(path: String) -> Result<DataFrame, String> {
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");
    // recupero il nome del primo sheet
    let name_first_sheet = workbook.sheet_names()[0].clone();
    // recupero lo sheet
    let sheet = match workbook.worksheet_range(name_first_sheet.as_str()) {
        Ok(sheet) => sheet,
        Err(e) => return Err(e.to_string()),
    };
    // estrapolo la riga di headers
    let headers: Vec<String> = sheet
        .rows()
        .nth(5)
        .unwrap()
        .iter()
        .map(|cell| cell.to_string().to_ascii_lowercase().replace(" ", "_"))
        .collect();
    // estrapolo tutti i dati e li salvo per colonne
    let mut column_data: Vec<Vec<String>> = vec![Vec::new(); headers.len()];
    for row in sheet.rows().skip(6).take(sheet.height() - (1 + 6)) {
        for (i, cell) in row.iter().enumerate() {
            if i < column_data.len() {
                column_data[i].push(cell.to_string());
            }
        }
    }
    // creo le colonne per il df dai dati recuperati
    let mut columns = Vec::new();
    for (i, header) in headers.iter().enumerate() {
        let series = Series::new(header.into(), &column_data[i]);
        columns.push(series.into());
    }
    // creazione del dataframe
    let df = DataFrame::new(columns).ok().unwrap();
    // seleziono dal df solo le colonne che mi interessano
    let cleaned_df = df
        .select([
            "fascicolo",
            "chiave",
            "nome_via",
            "piano",
            "id_spazio",
            "cod_stanza",
            "destinazione_uso",
        ])
        .unwrap();
    // elimino tutti i campi duplicati all'interno del df
    let unique_df = cleaned_df
        .unique_stable(None, UniqueKeepStrategy::First, None)
        .unwrap();
    Ok(unique_df)
}
