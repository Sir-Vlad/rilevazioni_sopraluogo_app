pub mod command_tauri {
    use crate::dao::{Edificio, EdificioDAO, EdificioDAOImpl, Stanza, StanzaDao, StanzaDaoImpl};
    use crate::database::{set_database, Database, DatabaseEventPayload};
    use crate::dto::{
        ClimatizzazioneDto, EdificioDTO, IlluminazioneDto, InfissoDto, MaterialeInfissoDto,
        StanzaDto, VetroInfissoDto,
    };
    use crate::service::{
        EdificioService, EdificioServiceImpl, ExportData, ExportDatiStanzaToExcel, InfissoService,
        InfissoServiceImpl, StanzaService, StanzaServiceImpl, TypeService, TypeServiceImpl,
    };
    use calamine::{open_workbook, Reader, Xlsx};
    use itertools::izip;
    use polars::frame::{DataFrame, UniqueKeepStrategy};
    use polars::prelude::{col, ChunkExplode, IntoLazy, NamedFrom, Series};
    use serde_json::Value;
    use std::collections::HashMap;
    use tauri::{AppHandle, Emitter, State};
    /**************************************************************************************************/
    /******************************* COMMAND PER GESTIRE IL DATABASE **********************************/
    /**************************************************************************************************/

    #[tauri::command]
    pub fn close_database(db: State<'_, Database>) -> Result<(), String> {
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

        db.with_transaction(|tx| {
            let df_cloned = df.clone().lazy();
            let grouped_addresses = df_cloned
                .group_by(["chiave"])
                .agg([col("nome_via").unique().explode()])
                .collect()
                .map_err(|e| e.to_string())?;
            let chiavi: Vec<&str> = grouped_addresses
                .column("chiave")
                .map_err(|err| err.to_string())?
                .str()
                .map_err(|err| err.to_string())?
                .into_iter()
                .flatten()
                .collect();
            let col_nome_via = grouped_addresses
                .column("nome_via")
                .map_err(|err| err.to_string())?;
            let indirizzi: Vec<String> = if col_nome_via.len() > 1 {
                col_nome_via
                    .as_series()
                    .unwrap()
                    .explode()
                    .map_err(|err| err.to_string())?
                    .explode()
                    .map_err(|err| err.to_string())?
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
                let edificio = Edificio::new(chiave.to_string(), fascicolo.clone(), indirizzo);
                EdificioDAOImpl::insert(tx, edificio)?;
            }

            for i in 0..df.height() {
                let stanza = Stanza::new(
                    retrieve_string_field_df(&df, "chiave", i)?,
                    retrieve_string_field_df(&df, "piano", i)?,
                    retrieve_string_field_df(&df, "id_spazio", i)?,
                    retrieve_string_field_df(&df, "cod_stanza", i)?,
                    retrieve_string_field_df(&df, "destinazione_uso", i)?,
                );
                StanzaDaoImpl::insert(tx, stanza)?;
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
    ) -> Result<String, String> {
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

    /**************************************************************************************************/
    /************************************** COMMAND PER INFISSI ***************************************/
    /**************************************************************************************************/

    #[tauri::command]
    pub fn get_infissi(db: State<'_, Database>) -> Result<Vec<InfissoDto>, String> {
        InfissoServiceImpl::get_all(db)
    }

    #[tauri::command]
    pub fn insert_infisso(
        db: State<'_, Database>,
        infisso: InfissoDto,
    ) -> Result<InfissoDto, String> {
        InfissoServiceImpl::insert(db, infisso)
    }

    #[tauri::command]
    pub fn update_infisso(
        db: State<'_, Database>,
        infisso: InfissoDto,
    ) -> Result<InfissoDto, String> {
        InfissoServiceImpl::update(db, infisso)
    }

    /**************************************************************************************************/
    /************************************** COMMAND PER STANZE ****************************************/
    /**************************************************************************************************/

    #[tauri::command]
    pub fn get_stanze(db: State<'_, Database>) -> Result<Vec<StanzaDto>, String> {
        StanzaServiceImpl::get_all(db)
    }

    #[tauri::command]
    pub fn insert_stanza(db: State<'_, Database>, stanza: StanzaDto) -> Result<StanzaDto, String> {
        StanzaServiceImpl::insert(db, stanza)
    }

    #[tauri::command]
    pub fn update_stanza(db: State<'_, Database>, stanza: StanzaDto) -> Result<StanzaDto, String> {
        StanzaServiceImpl::update(db, stanza)
    }

    #[tauri::command]
    pub fn get_infissi_stanza(db: State<'_, Database>, id: i64) -> Result<Vec<StanzaDto>, String> {
        StanzaServiceImpl::get_with_infissi(db, id)
    }

    #[tauri::command]
    pub fn insert_infissi_stanza(
        db: State<'_, Database>,
        stanza: StanzaDto,
    ) -> Result<StanzaDto, String> {
        StanzaServiceImpl::insert_with_infissi(db, stanza)
    }

    /**************************************************************************************************/
    /************************************** COMMAND PER TIPI ******************************************/
    /**************************************************************************************************/

    #[tauri::command]
    pub fn get_all_tipi(db: State<'_, Database>) -> Result<HashMap<String, Vec<Value>>, String> {
        TypeServiceImpl::get_all(db)
    }

    #[tauri::command]
    pub fn get_materiali_infisso(
        db: State<'_, Database>,
    ) -> Result<Vec<MaterialeInfissoDto>, String> {
        TypeServiceImpl::get_materiale_infisso(db)
    }

    #[tauri::command]
    pub fn get_vetro_infisso(db: State<'_, Database>) -> Result<Vec<VetroInfissoDto>, String> {
        TypeServiceImpl::get_vetro_infisso(db)
    }

    #[tauri::command]
    pub fn get_illuminazione(db: State<'_, Database>) -> Result<Vec<IlluminazioneDto>, String> {
        TypeServiceImpl::get_illuminazione(db)
    }

    #[tauri::command]
    pub fn get_climatizzazione(db: State<'_, Database>) -> Result<Vec<ClimatizzazioneDto>, String> {
        TypeServiceImpl::get_climatizzazione(db)
    }

    /**************************************************************************************************/
    /************************************** COMMAND PER EXPORT ******************************************/
    /**************************************************************************************************/

    #[tauri::command]
    pub fn export_data_to_excel(
        db: State<'_, Database>,
        name_file: Option<String>,
    ) -> Result<(), String> {
        ExportDatiStanzaToExcel::export(db, name_file)
    }

    /**************************************************************************************************/
    /***************************** COMMAND PER INIZIALIZZARE IL SISTEMA *******************************/
    /**************************************************************************************************/

    #[tauri::command]
    pub fn get_edifici(db: State<'_, Database>) -> Result<Vec<EdificioDTO>, String> {
        EdificioServiceImpl::get_all(db)
    }

    #[tauri::command]
    pub fn update_edificio(
        db: State<'_, Database>,
        edificio: EdificioDTO,
    ) -> Result<EdificioDTO, String> {
        EdificioServiceImpl::update(db, edificio)
    }
}
