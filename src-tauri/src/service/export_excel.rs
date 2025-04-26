use crate::database::Database;
use dirs_next::document_dir;
use rusqlite::{Connection, Error};
use rust_xlsxwriter::{Format, Workbook, Worksheet};
use serde::Serialize;
use std::fs;
use tauri::State;

#[derive(Debug, Serialize)]
pub struct DatiStanza {
    pub(crate) id: u64,
    pub(crate) fascicolo: String,
    pub(crate) chiave: String,
    pub(crate) piano: String,
    pub(crate) id_spazio: String,
    pub(crate) stanza: String,
    pub(crate) destinazione_uso: String,
    pub(crate) altezza: Option<u16>,
    pub(crate) spessore_muro: Option<u8>,
    pub(crate) riscaldamento: Option<String>,
    pub(crate) raffrescamento: Option<String>,
    pub(crate) illuminazione: Option<String>,
    pub(crate) mq_infissi: Option<f32>,
    pub(crate) materiale: Option<String>,
    pub(crate) vetro: Option<String>,
}

pub trait ExportData {
    fn export(db: State<'_, Database>, name_file: Option<String>) -> Result<(), String>;
}

pub struct ExportDatiStanzaToExcel;

impl ExportData for ExportDatiStanzaToExcel {
    fn export(db: State<'_, Database>, name_file: Option<String>) -> Result<(), String> {
        let conn = db.get_conn();
        if let Some(conn) = conn.as_ref() {
            let dati_stanze = Self::retrieve_data(conn)?;

            let mut workbook = Workbook::new();
            let worksheet = workbook.add_worksheet();
            worksheet.set_name("Dati").map_err(|e| e.to_string())?;

            // Scrittura dell'headers
            Self::write_headers(worksheet, Self::HEADERS)?;

            // Scrittura dei dati
            for (i, dato) in dati_stanze.as_ref().unwrap().iter().enumerate() {
                let row = (i + 1) as u32;
                Self::write_row(worksheet, row, dato)?
            }

            // Adatta le colonne
            worksheet.autofit();

            // Determina il percorso di salvataggio
            let doc_dir = document_dir().ok_or("Impossibile trovare la directory home")?;
            let mut export_path = doc_dir.join("Dati_Sopralluogo/export");
            if !export_path.exists() {
                match fs::create_dir_all(&export_path) {
                    Ok(()) => println!("Cartella {{export}} creata correttamente"),
                    Err(e) => return Err(format!("Errore nella creazione della cartella: {}", e)),
                }
            }

            export_path.push(format!(
                "{}.xlsx",
                match name_file {
                    Some(name) => name,
                    None => dati_stanze.ok().unwrap().first().unwrap().fascicolo.clone(),
                }
            ));

            // Salva il file
            workbook
                .save(export_path.to_str().unwrap())
                .map_err(|e| e.to_string())?;

            println!("File Excel esportato con successo!");

            Ok(())
        } else {
            Err("Errore durante la connessione al database".to_string())
        }
    }
}

impl ExportDatiStanzaToExcel {
    const HEADERS: [&'static str; 15] = [
        "ID",
        "Fascicolo",
        "Chiave",
        "Piano",
        "ID Spazio",
        "Stanza",
        "Destinazione Uso",
        "Altezza",
        "Spessore Muro",
        "Riscaldamento",
        "Raffrescamento",
        "Illuminazione",
        "MQ Infissi",
        "Materiale",
        "Vetro",
    ];
    fn retrieve_data(conn: &Connection) -> Result<Result<Vec<DatiStanza>, Error>, String> {
        let mut stmt = conn
            .prepare("SELECT * FROM DATI_STANZE")
            .map_err(|e| e.to_string())?;

        let dati_stanze: Result<Vec<DatiStanza>, Error> = stmt
            .query_map([], |row| {
                Ok(DatiStanza {
                    id: row.get(0)?,
                    fascicolo: row.get(1)?,
                    chiave: row.get(2)?,
                    piano: row.get(3)?,
                    id_spazio: row.get(4)?,
                    stanza: row.get(5)?,
                    destinazione_uso: row.get(6)?,
                    altezza: row.get(7)?,
                    spessore_muro: row.get(8)?,
                    riscaldamento: row.get(9)?,
                    raffrescamento: row.get(10)?,
                    illuminazione: row.get(11)?,
                    mq_infissi: row.get(12)?,
                    materiale: row.get(13)?,
                    vetro: row.get(14)?,
                })
            })
            .map_err(|e| e.to_string())?
            .collect();
        Ok(dati_stanze)
    }

    fn write_row(
        worksheet: &mut Worksheet,
        row: u32,
        row_value: &DatiStanza,
    ) -> Result<(), String> {
        let format = Format::new().set_num_format("0.00");
        worksheet
            .write_number(row, 0, row_value.id as f64)
            .map_err(|e| e.to_string())?;
        worksheet
            .write_string(row, 1, &row_value.fascicolo)
            .map_err(|e| e.to_string())?;
        worksheet
            .write_string(row, 2, &row_value.chiave)
            .map_err(|e| e.to_string())?;
        worksheet
            .write_string(row, 3, &row_value.piano)
            .map_err(|e| e.to_string())?;
        worksheet
            .write_string(row, 4, &row_value.id_spazio)
            .map_err(|e| e.to_string())?;
        worksheet
            .write_string(row, 5, &row_value.stanza)
            .map_err(|e| e.to_string())?;
        worksheet
            .write_string(row, 6, &row_value.destinazione_uso)
            .map_err(|e| e.to_string())?;
        if let Some(altezza) = row_value.altezza {
            worksheet
                .write_number(row, 7, altezza)
                .map_err(|e| e.to_string())?;
        }
        if let Some(altezza) = row_value.altezza {
            worksheet
                .write_number(row, 7, altezza)
                .map_err(|e| e.to_string())?;
        }
        if let Some(spessore) = row_value.spessore_muro {
            worksheet
                .write_number(row, 8, spessore)
                .map_err(|e| e.to_string())?;
        }
        if let Some(riscaldamento) = &row_value.riscaldamento {
            worksheet
                .write_string(row, 9, riscaldamento)
                .map_err(|e| e.to_string())?;
        }
        if let Some(raffrescamento) = &row_value.raffrescamento {
            worksheet
                .write_string(row, 10, raffrescamento)
                .map_err(|e| e.to_string())?;
        }
        if let Some(illuminazione) = &row_value.illuminazione {
            worksheet
                .write_string(row, 11, illuminazione)
                .map_err(|e| e.to_string())?;
        }
        if let Some(mq_infissi) = row_value.mq_infissi {
            worksheet
                .write_number_with_format(row, 12, mq_infissi, &format)
                .map_err(|e| e.to_string())?;
        }
        if let Some(materiale) = &row_value.materiale {
            worksheet
                .write_string(row, 13, materiale)
                .map_err(|e| e.to_string())?;
        }
        if let Some(vetro) = &row_value.vetro {
            worksheet
                .write_string(row, 14, vetro)
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    fn write_headers(worksheet: &mut Worksheet, headers: [&str; 15]) -> Result<(), String> {
        for (i, dato) in headers.iter().enumerate() {
            worksheet
                .write_string(0, i as u16, *dato)
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}
