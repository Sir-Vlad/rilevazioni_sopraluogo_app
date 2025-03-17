use calamine::{open_workbook, Reader, Xlsx};
use dirs_next::document_dir;
use polars::frame::DataFrame;
use polars::prelude::*;
use std::fs;
use std::fs::File;

const NAME_DIR_DATABASE: &str = "Dati_Sopralluogo";

#[tauri::command]
pub fn get_db_path(db_name: String) -> String {
    if let Some(mut path) = document_dir() {
        path.push(NAME_DIR_DATABASE);

        if !path.exists() {
            match std::fs::create_dir_all(&path) {
                Ok(()) => println!("Cartella creata correttamente"),
                Err(e) => println!("Errore nella creazione della cartella: {}", e),
            }
        }
        path.push(format!("{}.db", db_name));
        if !path.exists() {
            match File::create(&path) {
                Ok(_) => println!("Database creato correttamente"),
                Err(e) => println!("Errore nella creazione del database: {}", e),
            }
        }
        return path.to_string_lossy().to_string();
    }
    "Impossibile determinare la directory dei documenti".to_string()
}

#[tauri::command]
pub fn get_all_name_database() -> Result<Vec<String>, String> {
    if let Some(mut path) = document_dir() {
        path.push(NAME_DIR_DATABASE);

        if !path.exists() {
            return Err("Cartella non esiste".to_string());
        }

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

#[tauri::command]
pub fn elaborate_file(path: String) -> Result<String, String> {
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");
    let name_sheet = workbook.sheet_names()[0].clone();
    let sheet = match workbook.worksheet_range(name_sheet.as_str()) {
        Ok(sheet) => sheet,
        Err(e) => return Err(e.to_string()),
    };
    let headers: Vec<String> = sheet
        .rows()
        .nth(5)
        .unwrap()
        .iter()
        .map(|cell| cell.to_string().to_ascii_lowercase().replace(" ", "_"))
        .collect();

    let mut column_data: Vec<Vec<String>> = vec![Vec::new(); headers.len()];
    for row in sheet.rows().skip(8).take(sheet.height() - (1 + 8)) {
        for (i, cell) in row.iter().enumerate() {
            if i < column_data.len() {
                column_data[i].push(cell.to_string());
            }
        }
    }

    let mut columns = Vec::new();
    for (i, header) in headers.iter().enumerate() {
        let series = Series::new(header.into(), &column_data[i]);
        columns.push(series.into());
    }

    let df = DataFrame::new(columns).ok().unwrap();
    let cleaned_df = df
        .select([
            "fascicolo",
            "piano",
            "id_spazio",
            "cod_stanza",
            "destinazione_uso",
        ])
        .unwrap();
    let mut unique_df = cleaned_df
        .unique_stable(None, UniqueKeepStrategy::First, None)
        .unwrap();

    let mut buffer = Vec::new();
    JsonWriter::new(&mut buffer)
        .with_json_format(JsonFormat::Json)
        .finish(&mut unique_df)
        .expect("Errore nella creazione del json");

    let json_str = String::from_utf8(buffer).map_err(|e| e.to_string())?;
    Ok(json_str)
}

#[tauri::command]
pub fn create_new_file_database(name_file: String) -> Result<(), String> {
    println!("Creating database {}", name_file);
    if let Some(mut path) = document_dir() {
        path.push(NAME_DIR_DATABASE);
        if !path.exists() {
            match std::fs::create_dir_all(&path) {
                Ok(()) => println!("Cartella creata correttamente"),
                Err(e) => println!("Errore nella creazione della cartella: {}", e),
            }
        }
        path.push(format!("{}.db", name_file));
        if !path.exists() {
            match File::create(&path) {
                Ok(_) => println!("Database creato correttamente"),
                Err(e) => println!("Errore nella creazione del database: {}", e),
            }
        }
    }

    Ok(())
}
