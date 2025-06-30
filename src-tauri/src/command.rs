pub mod command_tauri {
    use crate::app_traits::{CreateService, RetrieveManyService, UpdateService};
    use crate::app_traits::{DtoTrait, Update};
    use crate::dao::StanzaDAO;
    use crate::dto::{
        AnnotazioneDTO, AnnotazioneEdificioDTO, AnnotazioneInfissoDTO, AnnotazioneStanzaDTO,
        FotovoltaicoDTO, TipoDTO, UtenzaDTO,
    };
    use crate::service::parser::{Command, InternalCommand, Parser, TypeFieldStanza, TypeValue};
    use crate::service::{
        import::ImportData, import::ImportDatiStanzaToExcel, AnnotazioneService, FotovoltaicoService,
        UtenzeService,
    };
    use crate::utils::{database_change_event, AppError};
    use crate::{
        db::{create_or_get_db_path, init_database, Database, NAME_DIR_DATABASE},
        dto::{EdificioDTO, InfissoDTO, StanzaDTO},
        service::{
            EdificioService, ExportData, ExportDatiStanzaToExcel, InfissoService, StanzaService,
            TypeService, TypeServiceImpl,
        },
    };
    use dirs_next::document_dir;
    use log::info;
    use serde_json::Value;
    use std::iter::repeat;
    use std::{collections::HashMap, ffi::OsStr, fs};
    use tauri::{AppHandle, State};

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

    pub fn set_database(
        app_handle: AppHandle,
        db: State<'_, Database>,
        db_name: String,
    ) -> ResultCommand<String> {
        let db_path = create_or_get_db_path(db_name).map_err(AppError::GenericError)?;
        db.switch_database(&db_path)?;
        db.with_transaction(|tx| {
            init_database(app_handle, tx)
                .map_err(|e| AppError::GenericError(format!("Init failed: {e}")))
        })?;
        Ok(db_path)
    }

    #[tauri::command]
    pub fn switch_database(
        app_handle: AppHandle,
        db: State<'_, Database>,
        db_name: String,
    ) -> ResultCommand<()> {
        info!("Switching db to {}", db_name);
        let db_path = create_or_get_db_path(db_name).map_err(AppError::GenericError)?;
        db.switch_database(&db_path)?;

        database_change_event(app_handle, db_path).map_err(|e| e.to_string())?;
        info!("Database switched");
        Ok(())
    }

    #[tauri::command]
    pub fn close_database(db: State<'_, Database>) -> ResultCommand<()> {
        db.close().map_err(|e| e.to_string())
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
        let df = ImportDatiStanzaToExcel::import(path)?;

        let name_db = df
            .column("fascicolo")
            .map_err(|e| e.to_string())?
            .get(0)
            .map_err(|e| e.to_string())?
            .to_string()
            .replace("\"", "");
        let db_path = set_database(app_handle.clone(), db.clone(), name_db)?;

        ImportDatiStanzaToExcel::save_to_database(db, df)?;

        // emit event del cambio di db
        database_change_event(app_handle, db_path.clone()).map_err(|e| e.to_string())?;
        Ok(db_path)
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

    #[tauri::command]
    pub fn insert_tipo(db: State<'_, Database>, tipo: TipoDTO) -> ResultCommand<TipoDTO> {
        TypeServiceImpl::insert_type(db, tipo).map_err(|e| e.to_string())
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
    }

    /**************************************************************************************************/
    /************************************ COMMAND PER UTENZE ****************************************/
    /**************************************************************************************************/

    #[tauri::command]
    pub fn get_annotazioni(
        db: State<'_, Database>,
        table: String,
    ) -> ResultCommand<Vec<AnnotazioneDTO>> {
        let result = match table.as_str() {
            "edificio" => handle_retrieve_many::<AnnotazioneEdificioDTO>(db)?,
            "stanza" => handle_retrieve_many::<AnnotazioneStanzaDTO>(db)?,
            "infisso" => handle_retrieve_many::<AnnotazioneInfissoDTO>(db)?,
            _ => {
                return Err(format!(
                    "Tabella {table} non ha le annotazioni",
                    table = table
                ));
            }
        };
        Ok(result)
    }

    #[tauri::command]
    pub fn insert_annotazione(
        db: State<'_, Database>,
        annotazione: AnnotazioneDTO,
    ) -> ResultCommand<AnnotazioneDTO> {
        let result = match annotazione.ref_table.as_str() {
            "edificio" => handle_create::<AnnotazioneEdificioDTO>(db, annotazione)?,
            "stanza" => handle_create::<AnnotazioneStanzaDTO>(db, annotazione)?,
            "infisso" => handle_create::<AnnotazioneInfissoDTO>(db, annotazione)?,
            _ => {
                return Err(format!(
                    "Tabella {table} non ha le annotazioni",
                    table = annotazione.ref_table
                ));
            }
        };
        Ok(result)
    }

    fn handle_retrieve_many<T>(db: State<'_, Database>) -> Result<Vec<AnnotazioneDTO>, AppError>
    where
        AnnotazioneService: RetrieveManyService<T>,
        T: Into<AnnotazioneDTO> + DtoTrait,
        AnnotazioneDTO: From<T>,
    {
        Ok(
            <AnnotazioneService as RetrieveManyService<T>>::retrieve_many(db)?
                .into_iter()
                .map(AnnotazioneDTO::from)
                .collect::<Vec<AnnotazioneDTO>>(),
        )
    }

    fn handle_create<T>(
        db: State<'_, Database>,
        annotazione: AnnotazioneDTO,
    ) -> Result<AnnotazioneDTO, AppError>
    where
        AnnotazioneService: CreateService<T>,
        T: From<AnnotazioneDTO> + DtoTrait,
        AnnotazioneDTO: From<T>,
    {
        Ok(<AnnotazioneService as CreateService<T>>::create(db, annotazione.into())?.into())
    }

    /**************************************************************************************************/
    /************************************ COMMAND PER QUERY *******************************************/
    /**************************************************************************************************/

    #[tauri::command]
    pub fn exec_command(db: State<'_, Database>, command: String) -> Result<(), AppError> {
        todo!("Funzione non implementata");
        let command_parsed = Command::parse(command.as_str())?;

        match command_parsed {
            Command::Internal(InternalCommand::Stanza(mut stanza_update)) => {
                let mut stanze: Vec<StanzaDTO> = Vec::with_capacity(stanza_update.stanze.len());

                for (stanza, stanza_update) in
                    stanze.iter_mut().zip(stanza_update.stanze.iter_mut())
                {
                    stanza.stanza = stanza_update.clone();
                }
                for (stanza, field_update) in
                    stanze.iter_mut().zip(stanza_update.fields_updates.iter())
                {
                    match field_update.field {
                        TypeFieldStanza::Altezza => {
                            stanza.altezza = Some(i64::try_from(&field_update.value)? as u16)
                        }
                        TypeFieldStanza::SpessoreMuro => {
                            stanza.spessore_muro = Some(i64::try_from(&field_update.value)? as u8)
                        }
                        TypeFieldStanza::Riscaldamento => {
                            stanza.riscaldamento = Some(String::try_from(&field_update.value)?)
                        }
                        TypeFieldStanza::Raffrescamento => {
                            stanza.raffrescamento = Some(String::try_from(&field_update.value)?)
                        }
                        TypeFieldStanza::Illuminazione => {
                            stanza.illuminazione = Some(String::try_from(&field_update.value)?)
                        }
                        TypeFieldStanza::Infissi => {
                            let value =
                                <&HashMap<String, TypeValue>>::try_from(&field_update.value)?;
                            let mut infissi_updated: Vec<String> = Vec::new();
                            for (k, v) in value {
                                let quantity = i64::try_from(v)?;
                                let i: Vec<String> =
                                    std::iter::repeat_n(k.clone(), quantity as usize).collect();
                                infissi_updated.extend(i);
                            }

                            stanza.infissi = Some(infissi_updated)
                        }
                    }
                }

                StanzaService::update_by_query(db, stanze)?;
            }
            Command::Internal(InternalCommand::Infisso()) => todo!(),
        }

        Ok(())
    }
}
