pub mod command_tauri {
    /*
        use crate::dto::{
            AnnotazioneDTO, AnnotazioneEdificioDTO, AnnotazioneInfissoDTO, AnnotazioneStanzaDTO,
            FotovoltaicoDTO, TipoDTO, UtenzaDTO,
        };
        use crate::service::{
            import::ImportData, import::ImportDatiStanzaToExcel, AnnotazioneService, CreateService,
            FotovoltaicoService, RetrieveManyService, UpdateService, UtenzeService,
        };
        use crate::utils::AppError;
        use crate::{
            database::{
                get_db_path, init_database, set_pragma, Database, DatabaseEventPayload,
                NAME_DIR_DATABASE,
            },
            dto::{EdificioDTO, InfissoDTO, StanzaDTO},
            service::{
                EdificioService, ExportData, ExportDatiStanzaToExcel, InfissoService, StanzaService,
                TypeService, TypeServiceImpl,
            },
        };
        use dirs_next::document_dir;
        use log::info;
        use rusqlite::Connection;
        use serde_json::Value;
        use std::{collections::HashMap, ffi::OsStr, fs};
        use tauri::{AppHandle, Emitter, State};
        use crate::constants::NAME_DIR_DATABASE;
        use crate::database::MigrationError::Connection;
    */
    type ResultCommand<T> = Result<T, String>;

    /*
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
    */
    use app_database::database::DatabaseManager;
    use tauri::State;

    #[tauri::command]
    pub fn close_database(db: State<'_, DatabaseManager>) -> ResultCommand<()> {
        Ok(())
    }
    /*
    /**************************************************************************************************/
    /***************************** COMMAND PER INIZIALIZZARE IL SISTEMA *******************************/
    /**************************************************************************************************/

    #[tauri::command]
    pub fn init_to_excel(
        app_handle: AppHandle,
        db: State<'_, Database>,
        path: String,
    ) -> ResultCommand<String> {
        let df = ImportDatiStanzaToExcel::import(path)?;

        let name_db = df
            .column("fascicolo")
            .map_err(|e| e.to_string())?
            .get(0)
            .map_err(|e| e.to_string())?
            .to_string()
            .replace("\"", "");
        let path_db = set_database(app_handle.clone(), db.clone(), name_db)?;

        ImportDatiStanzaToExcel::save_to_database(db, df)?;

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
    } /**************************************************************************************************/
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
    } /**************************************************************************************************/
    /************************************** COMMAND PER TIPI ******************************************/
    /**************************************************************************************************/
    #[tauri::command]
    pub fn get_all_tipi(db: State<'_, Database>) -> ResultCommand<HashMap<String, Vec<Value>>> {
        TypeServiceImpl::retrieve_all(db).map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub fn insert_tipo(db: State<'_, Database>, tipo: TipoDTO) -> ResultCommand<TipoDTO> {
        TypeServiceImpl::insert_type(db, tipo).map_err(|e| e.to_string())
    } /**************************************************************************************************/
    /************************************** COMMAND PER EXPORT ******************************************/
    /**************************************************************************************************/
    #[tauri::command]
    pub fn export_data_to_excel(
        db: State<'_, Database>,
        name_file: Option<String>,
    ) -> ResultCommand<()> {
        ExportDatiStanzaToExcel::export(db, name_file).map_err(|e| e.to_string())
    } /**************************************************************************************************/
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
    } /**************************************************************************************************/
    /************************************ COMMAND PER UTENZE ****************************************/
    /**************************************************************************************************/
    #[tauri::command]
    pub fn get_utenze(db: State<'_, Database>) -> ResultCommand<Vec<UtenzaDTO>> {
        UtenzeService::retrieve_many(db).map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub fn insert_utenza(db: State<'_, Database>, utenza: UtenzaDTO) -> ResultCommand<UtenzaDTO> {
        UtenzeService::create(db, utenza).map_err(|e| e.to_string())
    } /**************************************************************************************************/
    /******************************** COMMAND PER FOTOVOLTAICO ****************************************/
    /**************************************************************************************************/
    #[tauri::command]
    pub fn get_fotovoltaico(db: State<'_, Database>) -> ResultCommand<Vec<FotovoltaicoDTO>> {
        FotovoltaicoService::retrieve_many(db).map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub fn insert_fotovoltaico(
        db: State<'_, Database>,
        fotovoltaico: FotovoltaicoDTO,
    ) -> ResultCommand<FotovoltaicoDTO> {
        FotovoltaicoService::create(db, fotovoltaico).map_err(|e| e.to_string())
    } /**************************************************************************************************/
    /************************************ COMMAND PER UTENZE ****************************************/
    /**************************************************************************************************/
    #[tauri::command]
    pub fn get_annotazioni(
        db: State<'_, Database>,
        table: String,
    ) -> ResultCommand<Vec<AnnotazioneDTO>> {
        match table.as_str() {
            "edificio" => Ok(
                <AnnotazioneService as RetrieveManyService<AnnotazioneEdificioDTO>>::retrieve_many(
                    db,
                )
                .map_err(|e| e.to_string())?
                .into_iter()
                .map(AnnotazioneDTO::from)
                .collect::<Vec<AnnotazioneDTO>>(),
            ),
            "stanza" => Ok(
                <AnnotazioneService as RetrieveManyService<AnnotazioneStanzaDTO>>::retrieve_many(
                    db,
                )
                .map_err(|e| e.to_string())?
                .into_iter()
                .map(AnnotazioneDTO::from)
                .collect::<Vec<AnnotazioneDTO>>(),
            ),
            "infisso" => Ok(
                <AnnotazioneService as RetrieveManyService<AnnotazioneInfissoDTO>>::retrieve_many(
                    db,
                )
                .map_err(|e| e.to_string())?
                .into_iter()
                .map(AnnotazioneDTO::from)
                .collect::<Vec<AnnotazioneDTO>>(),
            ),
            _ => Err(format!(
                "Tabella {table} non ha le annotazioni",
                table = table
            )),
        }
    }

    #[tauri::command]
    pub fn insert_annotazione(
        db: State<'_, Database>,
        annotazione: AnnotazioneDTO,
    ) -> ResultCommand<AnnotazioneDTO> {
        match annotazione.ref_table.as_str() {
            "edificio" => Ok(
                <AnnotazioneService as CreateService<AnnotazioneEdificioDTO>>::create(
                    db,
                    annotazione.into(),
                )
                .map_err(|e| e.to_string())?
                .into(),
            ),
            "stanza" => Ok(
                <AnnotazioneService as CreateService<AnnotazioneStanzaDTO>>::create(
                    db,
                    annotazione.into(),
                )
                .map_err(|e| e.to_string())?
                .into(),
            ),
            "infisso" => Ok(
                <AnnotazioneService as CreateService<AnnotazioneInfissoDTO>>::create(
                    db,
                    annotazione.into(),
                )
                .map_err(|e| e.to_string())?
                .into(),
            ),
            _ => Err(format!(
                "Tabella {table} non ha le annotazioni",
                table = annotazione.ref_table
            )),
        }
    }
         */
}
