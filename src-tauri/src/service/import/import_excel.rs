use crate::dao::crud_operations::Insert;
use crate::dao::entity::{Edificio, Stanza};
use crate::dao::{EdificioDAO, StanzaDAO};
use crate::database::Database;
use crate::service::IdGeneratorStanza;
use crate::utils::AppError;
use calamine::{open_workbook, Reader, Xlsx};
use itertools::izip;
use polars::error::PolarsError;
use polars::frame::{DataFrame, UniqueKeepStrategy};
use polars::prelude::{col, ChunkExplode, IntoLazy, NamedFrom, Series};
use tauri::State;

pub trait ImportData {
    fn import(name_file: String) -> Result<DataFrame, AppError>;
    fn save_to_database(db: State<'_, Database>, df: DataFrame) -> Result<(), AppError>;
}

pub struct ImportDatiStanzaToExcel;

impl ImportData for ImportDatiStanzaToExcel {
    fn import(name_file: String) -> Result<DataFrame, AppError> {
        let mut workbook: Xlsx<_> = open_workbook(name_file).expect("File non trovato");
        // recupero il nome del primo sheet
        let name_first_sheet = workbook.sheet_names()[0].clone();
        // recupero lo sheet
        let sheet = match workbook.worksheet_range(name_first_sheet.as_str()) {
            Ok(sheet) => sheet,
            Err(e) => return Err(AppError::NotFound(e.to_string())),
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
        let cleaned_df = df.select([
            "fascicolo",
            "chiave",
            "nome_via",
            "piano",
            "id_spazio",
            "cod_stanza",
            "destinazione_uso",
        ])?;
        // elimino tutti i campi duplicati all'interno del df
        let unique_df = cleaned_df.unique_stable(None, UniqueKeepStrategy::First, None)?;
        Ok(unique_df)
    }

    fn save_to_database(db: State<'_, Database>, df: DataFrame) -> Result<(), AppError> {
        db.with_transaction(|tx| {
            let df_cloned = df.clone().lazy();
            let grouped_addresses = df_cloned
                .group_by(["chiave"])
                .agg([col("nome_via").unique().explode()])
                .collect()?;
            let chiavi: Vec<&str> = grouped_addresses
                .column("chiave")?
                .str()?
                .into_iter()
                .flatten()
                .collect();
            let indirizzi: Vec<String> = grouped_addresses
                .column("nome_via")?
                .list()
                .unwrap()
                .explode()
                .unwrap()
                .iter()
                .map(|el| el.get_str().unwrap().to_string())
                .collect();

            let fascicolo = Self::retrieve_string_field_df(&df, "fascicolo", 0)?;
            for (chiave, indirizzo) in izip!(chiavi, indirizzi) {
                let edificio = Edificio::new(chiave, fascicolo.as_str(), indirizzo.as_str());
                EdificioDAO::insert(tx, edificio)?;
            }

            let mut id_generator = IdGeneratorStanza::new();
            for i in 0..df.height() {
                let stanza = id_generator.generate_id(Stanza::new(
                    Self::retrieve_string_field_df(&df, "chiave", i)?.as_str(),
                    Self::retrieve_string_field_df(&df, "piano", i)?.as_str(),
                    Self::retrieve_string_field_df(&df, "id_spazio", i)?.as_str(),
                    Self::retrieve_string_field_df(&df, "cod_stanza", i)?.as_str(),
                    Self::retrieve_string_field_df(&df, "destinazione_uso", i)?.as_str(),
                ));
                StanzaDAO::insert(tx, stanza)?;
            }

            Ok(())
        })?;
        Ok(())
    }
}

impl ImportDatiStanzaToExcel {
    fn retrieve_string_field_df(
        df: &DataFrame,
        field: &str,
        index: usize,
    ) -> Result<String, PolarsError> {
        Ok(df.column(field)?.str()?.get(index).unwrap().to_string())
    }
}
