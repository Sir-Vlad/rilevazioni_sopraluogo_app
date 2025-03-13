use calamine::{open_workbook, Reader, Xlsx};
use dirs_next::document_dir;
use polars::frame::DataFrame;
use std::fs;
use std::fs::File;
use polars::prelude::*;

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
pub fn elaborate_file(path: String) -> Result<(), String> {
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");
    let sheet = workbook.worksheet_range("Sheet1").ok().unwrap();
    let headers: Vec<String> = sheet
        .rows()
        .next()
        .ok_or("Foglio vuoto")?
        .iter()
        .map(|cell| cell.to_string())
        .collect();

    let mut column_data: Vec<Vec<String>> = vec![Vec::new(); headers.len()];

    for row in sheet.rows().skip(1) {
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
    println!("{}", df);

    Ok(())
}
