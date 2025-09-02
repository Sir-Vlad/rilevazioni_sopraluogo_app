pub mod command_tauri {
    use app_services::{
        dto::{
            AnnotazioneDTO, AnnotazioneEdificioDTO, AnnotazioneInfissoDTO, AnnotazioneStanzaDTO,
            EdificioDTO, FotovoltaicoDTO, InfissoDTO, StanzaDTO, TipoDTO, UtenzaDTO,
        },
        service::{
            AnnotazioneService, CreateService, EdificioService, FotovoltaicoService,
            InfissoService, StanzaService, TypeService, TypeServiceImpl, UpdateService,
            UtenzeService,
        },
    };
    use app_state::database::DatabaseManager;
    use app_state::selected_edificio::StateEdificioSelected;
    use app_utils::app_interface::service_interface::RetrieveManyService;
    use serde_json::Value;
    use std::collections::HashMap;
    use tauri::State;

    type ResultCommand<T> = Result<T, String>;

    /**************************************************************************************************/
    /******************************* COMMAND PER MISCELLANEOUS **********************************/
    /**************************************************************************************************/

    #[tauri::command]
    pub async fn get_all_name_database(db: State<'_, DatabaseManager>) -> ResultCommand<Vec<String>> {
        let edifici = EdificioService::retrieve_many(db).await;
        match edifici {
            Ok(edifici) => {
                let res = edifici.iter().map(|edificio| edificio.fascicolo.to_string()).collect();
                Ok(res)
            }
            Err(_) => Ok(Vec::new()),
        }
    }
    /*
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
    */
    /**************************************************************************************************/
    /************************************** COMMAND PER INFISSI ***************************************/
    /**************************************************************************************************/

    #[tauri::command]
    pub async fn get_infissi(
        db: State<'_, DatabaseManager>,
        edificio_selected: State<'_, StateEdificioSelected>,
    ) -> ResultCommand<Vec<InfissoDTO>> {
        InfissoService::retrieve_infissi_by_edificio(db, edificio_selected)
            .await
            .map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn insert_infisso(
        db: State<'_, DatabaseManager>,
        infisso: InfissoDTO,
    ) -> ResultCommand<InfissoDTO> {
        InfissoService::create(db, infisso)
            .await
            .map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn update_infisso(
        db: State<'_, DatabaseManager>,
        infisso: InfissoDTO,
    ) -> ResultCommand<InfissoDTO> {
        InfissoService::update(db, infisso)
            .await
            .map_err(|e| e.to_string())
    }

    /**************************************************************************************************/
    /************************************** COMMAND PER STANZE ****************************************/
    /**************************************************************************************************/

    #[tauri::command]
    pub async fn get_stanze(
        db: State<'_, DatabaseManager>,
        edificio_selected: State<'_, StateEdificioSelected>,
    ) -> ResultCommand<Vec<StanzaDTO>> {
        StanzaService::get_stanze_edificio(db, edificio_selected)
            .await
            .map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn insert_stanza(
        db: State<'_, DatabaseManager>,
        stanza: StanzaDTO,
    ) -> ResultCommand<StanzaDTO> {
        StanzaService::create(db, stanza)
            .await
            .map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn update_stanza(
        db: State<'_, DatabaseManager>,
        stanza: StanzaDTO,
    ) -> ResultCommand<StanzaDTO> {
        StanzaService::update(db, stanza)
            .await
            .map_err(|e| e.to_string())
    }

    /**************************************************************************************************/
    /************************************** COMMAND PER TIPI ******************************************/
    /**************************************************************************************************/

    #[tauri::command]
    pub async fn get_all_tipi(
        db: State<'_, DatabaseManager>,
    ) -> ResultCommand<HashMap<String, Vec<Value>>> {
        TypeServiceImpl::retrieve_all(db)
            .await
            .map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn insert_tipo(
        db: State<'_, DatabaseManager>,
        tipo: TipoDTO,
    ) -> ResultCommand<TipoDTO> {
        TypeServiceImpl::insert_type(db, tipo)
            .await
            .map_err(|e| e.to_string())
    }

    /**************************************************************************************************/
    /************************************** COMMAND PER EXPORT ******************************************/
    /**************************************************************************************************/
    /*
    #[tauri::command]
    pub async fn export_data_to_excel(
        db: State<'_, DatabaseManager>,
        name_file: Option<String>,
    ) -> ResultCommand<()> {
        ExportDatiStanzaToExcel::export(db, name_file).map_err(|e| e.to_string())
    }
    */
    /**************************************************************************************************/
    /************************************ COMMAND PER EDIFICIO ****************************************/
    /**************************************************************************************************/
    #[tauri::command]
    pub async fn get_edifici(db: State<'_, DatabaseManager>) -> ResultCommand<Vec<EdificioDTO>> {
        EdificioService::retrieve_many(db)
            .await
            .map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn update_edificio(
        db: State<'_, DatabaseManager>,
        edificio: EdificioDTO,
    ) -> ResultCommand<EdificioDTO> {
        EdificioService::update(db, edificio)
            .await
            .map_err(|e| e.to_string())
    }

    /**************************************************************************************************/
    /************************************ COMMAND PER UTENZE ****************************************/
    /**************************************************************************************************/

    #[tauri::command]
    pub async fn get_utenze(db: State<'_, DatabaseManager>) -> ResultCommand<Vec<UtenzaDTO>> {
        UtenzeService::retrieve_many(db)
            .await
            .map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn insert_utenza(
        db: State<'_, DatabaseManager>,
        utenza: UtenzaDTO,
    ) -> ResultCommand<UtenzaDTO> {
        UtenzeService::create(db, utenza)
            .await
            .map_err(|e| e.to_string())
    }

    /**************************************************************************************************/
    /******************************** COMMAND PER FOTOVOLTAICO ****************************************/
    /**************************************************************************************************/
    #[tauri::command]
    pub async fn get_fotovoltaico(
        db: State<'_, DatabaseManager>,
    ) -> ResultCommand<Vec<FotovoltaicoDTO>> {
        FotovoltaicoService::retrieve_many(db)
            .await
            .map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn insert_fotovoltaico(
        db: State<'_, DatabaseManager>,
        fotovoltaico: FotovoltaicoDTO,
    ) -> ResultCommand<FotovoltaicoDTO> {
        FotovoltaicoService::create(db, fotovoltaico)
            .await
            .map_err(|e| e.to_string())
    }

    /**************************************************************************************************/
    /************************************ COMMAND PER UTENZE ****************************************/
    /**************************************************************************************************/
    #[tauri::command]
    pub async fn get_annotazioni(
        db: State<'_, DatabaseManager>,
        table: String,
    ) -> ResultCommand<Vec<AnnotazioneDTO>> {
        match table.as_str() {
            "edificio" => Ok(
                <AnnotazioneService as RetrieveManyService<AnnotazioneEdificioDTO>>::retrieve_many(
                    db,
                ).await
                    .map_err(|e| e.to_string())?
                    .into_iter()
                    .map(AnnotazioneDTO::from)
                    .collect::<Vec<AnnotazioneDTO>>(),
            ),
            "stanza" => Ok(
                <AnnotazioneService as RetrieveManyService<AnnotazioneStanzaDTO>>::retrieve_many(
                    db,
                )
                    .await
                    .map_err(|e| e.to_string())?
                    .into_iter()
                    .map(AnnotazioneDTO::from)
                    .collect::<Vec<AnnotazioneDTO>>(),
            ),
            "infisso" => Ok(
                <AnnotazioneService as RetrieveManyService<AnnotazioneInfissoDTO>>::retrieve_many(
                    db,
                )
                    .await
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
    pub async fn insert_annotazione(
        db: State<'_, DatabaseManager>,
        annotazione: AnnotazioneDTO,
    ) -> ResultCommand<AnnotazioneDTO> {
        match annotazione.get_ref_table() {
            "edificio" => Ok(
                <AnnotazioneService as CreateService<AnnotazioneEdificioDTO>>::create(
                    db,
                    annotazione.into(),
                )
                    .await
                    .map_err(|e| e.to_string())?
                    .into(),
            ),
            "stanza" => Ok(
                <AnnotazioneService as CreateService<AnnotazioneStanzaDTO>>::create(
                    db,
                    annotazione.into(),
                )
                    .await
                    .map_err(|e| e.to_string())?
                    .into(),
            ),
            "infisso" => Ok(
                <AnnotazioneService as CreateService<AnnotazioneInfissoDTO>>::create(
                    db,
                    annotazione.into(),
                )
                    .await
                    .map_err(|e| e.to_string())?
                    .into(),
            ),
            _ => Err(format!(
                "Tabella {table} non ha le annotazioni",
                table = annotazione.get_ref_table()
            )),
        }
    }
}
