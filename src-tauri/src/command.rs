pub mod command_tauri {
    use crate::dao::crud_operations::Insert;
    use crate::dto::UtenzaDTO;
    use crate::service::{CreateService, RetrieveManyService, UpdateService, UtenzeService};
    use crate::utils::AppError;
    use crate::{
        dao::{
            entity::{Edificio, Stanza},
            EdificioDAO, StanzaDAO,
        },
        database::{
            get_db_path, init_database, set_pragma, Database, DatabaseEventPayload,
            NAME_DIR_DATABASE,
        },
        dto::{
            ClimatizzazioneDTO, EdificioDTO, IlluminazioneDTO, InfissoDTO, MaterialeInfissoDTO,
            StanzaDTO, VetroInfissoDTO,
        },
        service::{
            EdificioService, ExportData, ExportDatiStanzaToExcel, InfissoService, StanzaService,
            TypeService, TypeServiceImpl,
        },
    };
    use calamine::{open_workbook, Reader, Xlsx};
    use dirs_next::document_dir;
    use itertools::izip;
    use log::info;
    use polars::prelude::PolarsError;
    use polars::{
        frame::{DataFrame, UniqueKeepStrategy},
        prelude::{col, ChunkExplode, IntoLazy, NamedFrom, Series},
    };
    use rusqlite::Connection;
    use serde_json::Value;
    use std::{collections::HashMap, ffi::OsStr, fs};
    use tauri::{AppHandle, Emitter, State};

    type ResultCommand<T> = Result<T, String>;

    /**************************************************************************************************/
    /******************************* COMMAND PER MISCELLANEOUS **********************************/
    /**************************************************************************************************/

    #[tauri::command]
    pub fn get_all_name_database() -> ResultCommand<Vec<String>> {
        if let Some(mut path) = document_dir() {
            path.push(NAME_DIR_DATABASE);
            if !path.exists() {
                return Err("Cartella non esiste".to_string());
            }
            // recupero tutti i nomi dei file all'interno della cartella
            let entries = fs::read_dir(path)
                .map_err(|e| AppError::GenericError(e.to_string()))?
                .filter_map(Result::ok)
                .filter(|entry| {
                    entry.path().is_file() && entry.path().extension() == Some(OsStr::new("db"))
                })
                .map(|entry| entry.file_name().to_string_lossy().into_owned())
                .collect::<Vec<String>>();
            return Ok(entries);
        }
        Err("La cartella Documenti non è stata trovata".to_string())
    }

    /**************************************************************************************************/
    /******************************* COMMAND PER GESTIRE IL DATABASE **********************************/
    /**************************************************************************************************/

    #[tauri::command]
    pub fn set_database(
        app_handle: AppHandle,
        db: State<'_, Database>,
        db_name: String,
    ) -> ResultCommand<String> {
        let db_path = get_db_path(db_name).map_err(AppError::GenericError)?;
        {
            let mut conn = db.get_conn();
            let mut path_to_database = db.get_path_to_database();
            if let Some(existing_conn) = conn.take() {
                drop(existing_conn);
            }
            *conn = Some(Connection::open(&db_path).map_err(|e| e.to_string())?);
            *path_to_database = Some(db_path.clone());

            set_pragma(conn.as_ref().unwrap()).map_err(|e| e.to_string())?;
        } // unlock mutex
        db.with_transaction(|tx| init_database(app_handle, tx).map_err(AppError::GenericError))?;
        Ok(db_path)
    }

    #[tauri::command]
    pub fn switch_database(
        app_handle: AppHandle,
        db: State<'_, Database>,
        db_name: String,
    ) -> ResultCommand<()> {
        info!("Switching database to {}", db_name);
        let db_path = get_db_path(db_name).map_err(AppError::GenericError)?;
        let mut conn = db.get_conn();
        let mut path_to_database = db.get_path_to_database();
        if let Some(existing_conn) = conn.take() {
            drop(existing_conn);
        }
        *conn = Some(Connection::open(&db_path).map_err(|e| e.to_string())?);
        *path_to_database = Some(db_path.clone());
        set_pragma(conn.as_ref().unwrap()).map_err(|e| e.to_string())?;

        app_handle
            .emit(
                "database-changed",
                DatabaseEventPayload {
                    type_event: "database_switched",
                    path: db_path.clone(),
                },
            )
            .map_err(|e| e.to_string())?;

        info!("Database switched");
        Ok(())
    }

    #[tauri::command]
    pub fn close_database(db: State<'_, Database>) -> ResultCommand<()> {
        let mut conn = db.get_conn();
        if let Some(conn) = conn.take() {
            drop(conn);
        }
        *conn = None;
        let mut path_database = db.get_path_to_database();
        if let Some(path) = path_database.take() {
            drop(path);
        }
        *path_database = None;
        Ok(())
    }

    /**************************************************************************************************/
    /***************************** COMMAND PER INIZIALIZZARE IL SISTEMA *******************************/
    /**************************************************************************************************/

    #[tauri::command]
    pub fn init_to_excel(
        app_handle: AppHandle,
        db: State<'_, Database>,
        path: String,
    ) -> ResultCommand<String> {
        let df = elaborate_file(path)?;

        let name_db = df
            .column("fascicolo")
            .map_err(|e| e.to_string())?
            .get(0)
            .map_err(|e| e.to_string())?
            .to_string()
            .replace("\"", "");
        let path_db = set_database(app_handle.clone(), db.clone(), name_db)?;

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
            let col_nome_via = grouped_addresses.column("nome_via")?;
            let indirizzi: Vec<String> = if col_nome_via.len() > 1 {
                col_nome_via
                    .as_series()
                    .unwrap()
                    .explode()?
                    .explode()?
                    .iter()
                    .map(|el| el.get_str().unwrap().to_string())
                    .collect()
            } else {
                col_nome_via
                    .list()
                    .unwrap()
                    .explode()
                    .unwrap()
                    .iter()
                    .map(|el| el.get_str().unwrap().to_string())
                    .collect()
            };

            let fascicolo = retrieve_string_field_df(&df, "fascicolo", 0)?;
            for (chiave, indirizzo) in izip!(chiavi, indirizzi) {
                let edificio = Edificio::new(chiave, fascicolo.as_str(), indirizzo.as_str());
                EdificioDAO::insert(tx, edificio)?;
            }

            for i in 0..df.height() {
                let stanza = Stanza::new(
                    retrieve_string_field_df(&df, "chiave", i)?.as_str(),
                    retrieve_string_field_df(&df, "piano", i)?.as_str(),
                    retrieve_string_field_df(&df, "id_spazio", i)?.as_str(),
                    retrieve_string_field_df(&df, "cod_stanza", i)?.as_str(),
                    retrieve_string_field_df(&df, "destinazione_uso", i)?.as_str(),
                );
                StanzaDAO::insert(tx, stanza)?;
            }

            Ok(())
        })?;

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
    }

    fn retrieve_string_field_df(
        df: &DataFrame,
        field: &str,
        index: usize,
    ) -> Result<String, PolarsError> {
        Ok(df.column(field)?.str()?.get(index).unwrap().to_string())
    }

    fn elaborate_file(path: String) -> Result<DataFrame, AppError> {
        let mut workbook: Xlsx<_> = open_workbook(path).expect("File non trovato");
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

    /**************************************************************************************************/
    /************************************** COMMAND PER INFISSI ***************************************/
    /**************************************************************************************************/

    #[tauri::command]
    pub fn get_infissi(db: State<'_, Database>) -> ResultCommand<Vec<InfissoDTO>> {
        InfissoService::retrieve_many(db).map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub fn insert_infisso(
        db: State<'_, Database>,
        infisso: InfissoDTO,
    ) -> ResultCommand<InfissoDTO> {
        InfissoService::create(db, infisso).map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub fn update_infisso(
        db: State<'_, Database>,
        infisso: InfissoDTO,
    ) -> ResultCommand<InfissoDTO> {
        InfissoService::update(db, infisso).map_err(|e| e.to_string())
    }

    /**************************************************************************************************/
    /************************************** COMMAND PER STANZE ****************************************/
    /**************************************************************************************************/

    #[tauri::command]
    pub fn get_stanze(db: State<'_, Database>) -> ResultCommand<Vec<StanzaDTO>> {
        StanzaService::retrieve_many(db).map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub fn insert_stanza(db: State<'_, Database>, stanza: StanzaDTO) -> ResultCommand<StanzaDTO> {
        StanzaService::create(db, stanza).map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub fn update_stanza(db: State<'_, Database>, stanza: StanzaDTO) -> ResultCommand<StanzaDTO> {
        StanzaService::update(db, stanza).map_err(|e| e.to_string())
    }

    /**************************************************************************************************/
    /************************************** COMMAND PER TIPI ******************************************/
    /**************************************************************************************************/

    #[tauri::command]
    pub fn get_all_tipi(db: State<'_, Database>) -> ResultCommand<HashMap<String, Vec<Value>>> {
        TypeServiceImpl::retrieve_all(db).map_err(|e| e.to_string())
    }

    /**************************************************************************************************/
    /************************************** COMMAND PER EXPORT ******************************************/
    /**************************************************************************************************/

    #[tauri::command]
    pub fn export_data_to_excel(
        db: State<'_, Database>,
        name_file: Option<String>,
    ) -> ResultCommand<()> {
        ExportDatiStanzaToExcel::export(db, name_file).map_err(|e| e.to_string())
    }

    /**************************************************************************************************/
    /************************************ COMMAND PER EDIFICIO ****************************************/
    /**************************************************************************************************/

    #[tauri::command]
    pub fn get_edifici(db: State<'_, Database>) -> ResultCommand<Vec<EdificioDTO>> {
        EdificioService::retrieve_many(db).map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub fn update_edificio(
        db: State<'_, Database>,
        edificio: EdificioDTO,
    ) -> ResultCommand<EdificioDTO> {
        EdificioService::update(db, edificio).map_err(|e| e.to_string())
    }

    /**************************************************************************************************/
    /************************************ COMMAND PER UTENZE ****************************************/
    /**************************************************************************************************/

    #[tauri::command]
    pub fn get_utenze(db: State<'_, Database>) -> ResultCommand<Vec<UtenzaDTO>> {
        UtenzeService::retrieve_many(db).map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub fn insert_utenza(db: State<'_, Database>, utenza: UtenzaDTO) -> ResultCommand<UtenzaDTO> {
        UtenzeService::create(db, utenza).map_err(|e| e.to_string())
    }

    /**************************************************************************************************/
    /************************************ COMMAND PER UTENZE ****************************************/
    /**************************************************************************************************/

    #[tauri::command]
    pub fn get_fotovoltaico(db: State<'_, Database>) -> ResultCommand<Vec<UtenzaDTO>> {
        UtenzeService::retrieve_many(db).map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub fn insert_fotovoltaico(
        db: State<'_, Database>,
        fotovoltaico: UtenzaDTO,
    ) -> ResultCommand<UtenzaDTO> {
        UtenzeService::create(db, fotovoltaico).map_err(|e| e.to_string())
    }
}
