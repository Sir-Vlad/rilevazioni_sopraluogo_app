use crate::app_traits::GetAll;
use crate::dao::crud_operations::GetAll as Gold;
use crate::dao::entity::{DatiStanza, Edificio, Fotovoltaico};
use crate::dao::{DatiStanzeViewDAO, EdificioDAO, FotovoltaicoDAO};
use crate::database::Database;
use crate::utils::{AppError, ToList};
use dirs_next::document_dir;
use log::info;
use rust_xlsxwriter::{ColNum, Format, RowNum, Workbook, Worksheet};
use std::any::{Any, TypeId};
use std::fs;
use tauri::State;

pub trait ExportData {
    fn export(db: State<'_, Database>, name_file: Option<String>) -> Result<(), String>;
}

pub struct ExportDatiStanzaToExcel;

impl ExportData for ExportDatiStanzaToExcel {
    fn export(db: State<'_, Database>, name_file: Option<String>) -> Result<(), String> {
        let conn = db.get_conn()?;
        if let Some(conn) = conn.as_ref() {
            let dati_stanze = DatiStanzeViewDAO::get_all(conn)?;

            let mut workbook = Workbook::new();
            // Primo worksheet
            let worksheet = workbook.add_worksheet();
            worksheet.set_name("Stanze").map_err(|e| e.to_string())?;

            // Scrittura dell'headers
            Self::write_headers(
                worksheet,
                DatiStanza::get_fields()
                    .iter()
                    .map(|x| x.to_string())
                    .collect(),
            )?;

            // Scrittura dei dati
            for (i, dato) in dati_stanze.iter().enumerate() {
                let row = (i + 1) as u32;
                Self::write_row(worksheet, row, &dato.to_list())?
            }

            // Adatta le colonne
            worksheet.autofit();

            // Secondo worksheet
            let worksheet = workbook.add_worksheet();
            worksheet.set_name("Edifici").map_err(|e| e.to_string())?;

            // headers
            let f_headers = Fotovoltaico::get_fields();
            let mut headers: Vec<String> = Edificio::get_fields();
            headers.extend(vec![f_headers[2].clone(), f_headers[3].clone()]);
            Self::write_headers(worksheet, headers)?;

            // Dati
            let edifici = EdificioDAO::get_all(conn)?;
            let fotovoltaico = FotovoltaicoDAO::get_all(conn)?;
            for (index, edificio) in edifici.iter().enumerate() {
                let f = fotovoltaico
                    .iter()
                    .find(|&f| f.id_edificio == edificio.chiave);
                let edificio = edificio.to_list();
                Self::write_row(worksheet, (index + 1) as RowNum, &edificio)?;
                if let Some(f) = f {
                    let f_list = f.to_list();
                    Self::write_cell(
                        worksheet,
                        0,
                        (edificio.len() + 1) as ColNum,
                        f_list.get(2).unwrap(),
                    )?;
                    Self::write_cell(
                        worksheet,
                        0,
                        (edificio.len() + 2) as ColNum,
                        f_list.get(3).unwrap(),
                    )?;
                }
            }

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
                    None => dati_stanze.first().unwrap().fascicolo.clone(),
                }
            ));

            // Salva il file
            workbook
                .save(export_path.to_str().unwrap())
                .map_err(|e| e.to_string())?;

            info!("File Excel esportato con successo!");

            Ok(())
        } else {
            Err(AppError::DatabaseNotInitialized.to_string())
        }
    }
}

impl ExportDatiStanzaToExcel {
    fn write_headers(worksheet: &mut Worksheet, fields: Vec<String>) -> Result<(), String> {
        let format = Format::new().set_bold();

        for (i, dato) in fields.iter().enumerate() {
            worksheet
                .write_string_with_format(0, i as u16, dato, &format)
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    fn write_row(
        worksheet: &mut Worksheet,
        row: RowNum,
        row_value: &Vec<Box<dyn Any>>,
    ) -> Result<(), String> {
        for (index, value) in row_value.iter().enumerate() {
            Self::write_cell(worksheet, row, index as ColNum, value)?;
        }
        Ok(())
    }

    fn write_cell(
        worksheet: &mut Worksheet,
        row: RowNum,
        col: ColNum,
        value: &Box<dyn Any>,
    ) -> Result<(), String> {
        let format = Format::new().set_num_format("0.00");
        let type_id = (**value).type_id();

        match type_id {
            id if id == TypeId::of::<String>() => {
                let value = value.downcast_ref::<String>().unwrap();
                Self::write_cell_string(worksheet, row, col, value)?;
            }
            id if id == TypeId::of::<u64>() => {
                let value = value.downcast_ref::<u64>().unwrap();
                Self::write_cell_number(worksheet, row, col, *value as f64)?;
            }
            id if id == TypeId::of::<Option<u16>>() || id == TypeId::of::<Option<u8>>() => {
                let value_u16 = if id == TypeId::of::<Option<u8>>() {
                    Self::try_convert_option_number::<u8, u16>(value).unwrap()
                } else {
                    *value.downcast_ref::<Option<u16>>().unwrap()
                };

                match value_u16 {
                    Some(value) => Self::write_cell_number(worksheet, row, col, value as f64)?,
                    None => Self::write_cell_number(worksheet, row, col, 0.0)?,
                };
            }
            id if id == TypeId::of::<Option<String>>() => {
                let value = value.downcast_ref::<Option<String>>().unwrap();
                if let Some(value) = value.clone() {
                    Self::write_cell_string(worksheet, row, col, value.as_str())?;
                }
            }
            id if id == TypeId::of::<Option<f32>>() => {
                match value.downcast_ref::<Option<f32>>().unwrap() {
                    Some(value) => Self::write_cell_number_with_format(
                        worksheet,
                        row,
                        col,
                        *value as f64,
                        &format,
                    )?,
                    None => Self::write_cell_number_with_format(worksheet, row, col, 0.0, &format)?,
                };
            }
            id if id == TypeId::of::<Option<bool>>() => {
                match value.downcast_ref::<Option<bool>>().unwrap() {
                    Some(value) => Self::write_cell_string(
                        worksheet,
                        row,
                        col,
                        if *value { "SI" } else { "NO" },
                    )?,
                    None => Self::write_cell_string(worksheet, row, col, "NO")?,
                };
            }
            _ => {} // Gestione per tipi non supportati
        }
        Ok(())
    }

    fn try_convert_option_number<From: Any + Copy, To: TryFrom<From>>(
        value: &Box<dyn Any>,
    ) -> Option<Option<To>>
    where
        <To as TryFrom<From>>::Error: std::fmt::Debug,
    {
        if let Some(opt_from) = value.downcast_ref::<Option<From>>() {
            return Some(opt_from.map(|val| To::try_from(val).unwrap()));
        }
        None
    }

    fn write_cell_string(
        worksheet: &mut Worksheet,
        row: RowNum,
        col: ColNum,
        value: &str,
    ) -> Result<(), String> {
        worksheet
            .write_string(row, col, value)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    fn write_cell_number(
        worksheet: &mut Worksheet,
        row: RowNum,
        col: ColNum,
        value: f64,
    ) -> Result<(), String> {
        worksheet
            .write_number(row, col, value)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    fn write_cell_number_with_format(
        worksheet: &mut Worksheet,
        row: RowNum,
        col: ColNum,
        value: f64,
        format: &Format,
    ) -> Result<(), String> {
        worksheet
            .write_number_with_format(row, col, value, format)
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
