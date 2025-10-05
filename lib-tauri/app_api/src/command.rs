use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use app_data_processing::{IdGeneratorStanza, SimpleDataFrame, TransposedDataFrame};
use app_services::{
    dto::{
        AnnotazioneDTO, AnnotazioneEdificioDTO, AnnotazioneInfissoDTO, AnnotazioneStanzaDTO,
        EdificioDTO, FotovoltaicoDTO, InfissoDTO, StanzaDTO, StanzaDTOBuilder, TableWithPrimaryKey,
        TipoDTO, UtenzaDTO,
    },
    service::{
        AnnotazioneService, CreateService, EdificioService, FotovoltaicoService, InfissoService,
        StanzaService, TypeService, TypeServiceImpl, UpdateService, UtenzeService,
    },
};
use app_state::{database::DatabaseManager, selected_edificio::EdificioSelected};
use app_utils::app_interface::service_interface::{
    CreateBatchService, RetrieveByEdificioSelected, RetrieveManyService, SelectedEdificioState,
    SelectedEdificioTrait,
};
use log::info;
use serde_json::Value;
use tauri::{AppHandle, Emitter, Runtime, State};

use crate::{
    events_payload::{EdificioChangePayload, EventWrapper, NewEdificioPayload, TypeEvent},
    get_chiave_selected_edificio, is_selected_edificio,
};

pub(crate) type ResultCommand<T> = Result<T, String>;

/***************************************************************************
 * ********************** */
/******************************* COMMAND PER MISCELLANEOUS
 * ********************************* */
/***************************************************************************
 * ********************** */

#[tauri::command]
pub async fn get_fascicoli(db: State<'_, DatabaseManager>) -> ResultCommand<Vec<i32>> {
    let edifici = EdificioService::retrieve_many(db).await;
    match edifici {
        Ok(edifici) => {
            let unique_fascicoli: HashSet<i32> =
                edifici.iter().map(|edificio| edificio.fascicolo).collect();
            let mut res = unique_fascicoli.into_iter().collect::<Vec<i32>>();
            res.sort();
            Ok(res)
        }
        Err(_) => Ok(Vec::new()),
    }
}

/***************************************************************************
 * ********************** */
/******************************* COMMAND PER GESTIRE IL DATABASE
 * ********************************* */
/***************************************************************************
 * ********************** */

#[tauri::command]
pub async fn set_edificio(
    app_handle: AppHandle,
    edificio_selected: State<'_, SelectedEdificioState<EdificioSelected>>,
    chiave: String,
) -> ResultCommand<()> {
    info!("Switching edificio to {}", chiave);
    EdificioService::select_edificio(edificio_selected.clone(), chiave).await;

    app_handle
        .emit(
            "edificio",
            EventWrapper::new(
                TypeEvent::ChangedEdificio,
                EdificioChangePayload::new(
                    get_chiave_selected_edificio(edificio_selected)
                        .await
                        .unwrap(),
                ),
            ),
        )
        .map_err(|e| e.to_string())?;

    info!("Edificio switched");
    Ok(())
}

#[tauri::command]
pub async fn clear_edificio(
    edificio_selected: State<'_, SelectedEdificioState<EdificioSelected>>,
) -> ResultCommand<()> {
    EdificioService::clear_edificio(edificio_selected).await;
    info!("Edificio cleared");
    Ok(())
}

/***************************************************************************
 * ********************** */
/************************** COMMAND PER AGGIUNGERE UN NUOVO FASCICOLO
 * **************************** */
/***************************************************************************
 * ********************** */

fn get_field(row: &HashMap<&str, &str>, name: &str) -> Result<String, Box<dyn Error>> {
    match row.get(name) {
        Some(chiave) => Ok(chiave.to_string()),
        None => Err(Box::from(format!("Campo {name} non trovato"))),
    }
}

pub async fn save_to_database(
    db: State<'_, DatabaseManager>,
    df: TransposedDataFrame,
) -> Result<Vec<EdificioDTO>, Box<dyn Error>> {
    let df_clone = {
        let mut df = df.clone().traspose();
        df.select(&["chiave", "fascicolo", "nome_via"]).ok();
        df.unique();
        df.traspose()
    };
    let mut result = Vec::new();
    for row in df_clone.iter_rows() {
        let new_edificio = EdificioDTO {
            chiave: get_field(&row, "chiave")?,
            fascicolo: get_field(&row, "fascicolo")?.parse()?,
            indirizzo: get_field(&row, "nome_via")?,
            anno_costruzione: None,
            anno_riqualificazione: None,
            note_riqualificazione: None,
            isolamento_tetto: false,
            cappotto: false,
        };
        result.push(EdificioService::create(db.clone(), new_edificio).await?);
    }

    let mut generator_id_stanza = IdGeneratorStanza::new();
    let mut new_stanze: Vec<StanzaDTO> = Vec::new();
    for row in df.iter_rows() {
        let new_stanza: StanzaDTO = generator_id_stanza
            .generate_id(
                StanzaDTOBuilder::default()
                    .edificio_id(get_field(&row, "chiave")?)
                    .piano(get_field(&row, "piano")?)
                    .id_spazio(get_field(&row, "id_spazio")?)
                    .cod_stanza(get_field(&row, "cod_stanza")?)
                    .destinazione_uso(get_field(&row, "destinazione_uso")?)
                    .build()
                    .into(),
            )?
            .into();
        new_stanze.push(new_stanza);
    }
    StanzaService::create_batch(db.clone(), new_stanze).await?;
    Ok(result)
}

#[tauri::command]
pub async fn add_new_fascicolo_from_xlsx<R: Runtime>(
    app_handle: AppHandle<R>,
    db: State<'_, DatabaseManager>,
    selected_edificio: State<'_, SelectedEdificioState<EdificioSelected>>,
    path: String,
) -> ResultCommand<()> {
    let df = SimpleDataFrame::from_xlsx(path.as_str()).map_err(|e| e.to_string())?;
    let chiavi = df.column("chiave").map_err(|e| e.to_string())?;
    let first_chiave = chiavi.first().ok_or(Box::from("Chiave non trovato"))?;

    let new_edifici = save_to_database(db, df.traspose())
        .await
        .map_err(|e| e.to_string())?;

    selected_edificio
        .write()
        .await
        .set_chiave(first_chiave.clone());

    // emit event del cambio di database
    app_handle
        .emit(
            "edificio",
            EventWrapper::new(
                TypeEvent::NewEdificio,
                NewEdificioPayload::new(
                    new_edifici,
                    get_chiave_selected_edificio(selected_edificio)
                        .await
                        .unwrap(),
                ),
            ),
        )
        .map_err(|e| e.to_string())?;

    Ok(())
}

/***************************************************************************
 * ********************** */
/************************************** COMMAND PER INFISSI
 * ************************************** */
/***************************************************************************
 * ********************** */

#[tauri::command]
pub async fn get_infissi(
    db: State<'_, DatabaseManager>,
    edificio_selected: State<'_, SelectedEdificioState<EdificioSelected>>,
) -> ResultCommand<Vec<InfissoDTO>> {
    InfissoService::retrieve_by_edificio_selected(db, edificio_selected)
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

/***************************************************************************
 * ********************** */
/************************************** COMMAND PER STANZE
 * *************************************** */
/***************************************************************************
 * ********************** */

#[tauri::command]
pub async fn get_stanze(
    db: State<'_, DatabaseManager>,
    edificio_selected: State<'_, SelectedEdificioState<EdificioSelected>>,
) -> ResultCommand<Vec<StanzaDTO>> {
    StanzaService::retrieve_by_edificio_selected(db, edificio_selected)
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

/***************************************************************************
 * ********************** */
/************************************** COMMAND PER TIPI
 * ***************************************** */
/***************************************************************************
 * ********************** */

#[tauri::command]
pub async fn get_all_tipi(
    db: State<'_, DatabaseManager>,
) -> ResultCommand<HashMap<String, Vec<Value>>> {
    TypeServiceImpl::retrieve_all(db)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn insert_tipo(db: State<'_, DatabaseManager>, tipo: TipoDTO) -> ResultCommand<TipoDTO> {
    TypeServiceImpl::insert_type(db, tipo)
        .await
        .map_err(|e| e.to_string())
}

/***************************************************************************
 * ********************** */
/************************************** COMMAND PER EXPORT
 * ***************************************** */
/***************************************************************************
 * ********************** */
/*
#[tauri::command]
pub async fn export_data_to_excel(
db: State<'_, DatabaseManager>,
name_file: Option<String>,
) -> ResultCommand<()> {
ExportDatiStanzaToExcel::export(db, name_file).map_err(|e| e.to_string())
}
*/
/***************************************************************************
 * ********************** */
/************************************ COMMAND PER EDIFICIO
 * *************************************** */
/***************************************************************************
 * ********************** */
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

/***************************************************************************
 * ********************** */
/************************************ COMMAND PER UTENZE
 * *************************************** */
/***************************************************************************
 * ********************** */

#[tauri::command]
pub async fn get_utenze(
    db: State<'_, DatabaseManager>,
    selected_edificio: State<'_, SelectedEdificioState<EdificioSelected>>,
) -> ResultCommand<Vec<UtenzaDTO>> {
    if !is_selected_edificio(selected_edificio.clone()).await {
        return Ok(Vec::new());
    }

    UtenzeService::retrieve_by_edificio_selected(db, selected_edificio)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn insert_utenza(
    db: State<'_, DatabaseManager>,
    selected_edificio: State<'_, SelectedEdificioState<EdificioSelected>>,
    utenza: UtenzaDTO,
) -> ResultCommand<UtenzaDTO> {
    if !is_selected_edificio(selected_edificio.clone()).await {
        return Err("Non selezionato un edificio".to_string());
    }

    if get_chiave_selected_edificio(selected_edificio)
        .await
        .unwrap()
        != utenza.edificio_id
    {
        return Err("Chiave non corrispondente".to_string());
    }

    UtenzeService::create(db, utenza)
        .await
        .map_err(|e| e.to_string())
}

/***************************************************************************
 * ********************** */
/******************************** COMMAND PER FOTOVOLTAICO
 * *************************************** */
/***************************************************************************
 * ********************** */
#[tauri::command]
pub async fn get_fotovoltaico(
    db: State<'_, DatabaseManager>,
    selected_edificio: State<'_, SelectedEdificioState<EdificioSelected>>,
) -> ResultCommand<Vec<FotovoltaicoDTO>> {
    if !is_selected_edificio(selected_edificio.clone()).await {
        return Ok(Vec::new());
    }

    FotovoltaicoService::retrieve_by_edificio_selected(db, selected_edificio)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn insert_fotovoltaico(
    db: State<'_, DatabaseManager>,
    selected_edificio: State<'_, SelectedEdificioState<EdificioSelected>>,
    fotovoltaico: FotovoltaicoDTO,
) -> ResultCommand<FotovoltaicoDTO> {
    if !is_selected_edificio(selected_edificio.clone()).await {
        return Err("Non selezionato un edificio".to_string());
    }

    if get_chiave_selected_edificio(selected_edificio)
        .await
        .unwrap()
        != fotovoltaico.id_edificio
    {
        return Err("Chiave non corrispondente".to_string());
    }

    FotovoltaicoService::create(db, fotovoltaico)
        .await
        .map_err(|e| e.to_string())
}

/***************************************************************************
 * ********************** */
/********************************** COMMAND PER ANNOTAZIONI
 * ************************************** */
/***************************************************************************
 * ********************** */
#[tauri::command]
pub async fn get_annotazioni(
    db: State<'_, DatabaseManager>,
    table: TableWithPrimaryKey,
) -> ResultCommand<Vec<AnnotazioneDTO>> {
    match table {
        TableWithPrimaryKey::Edificio(..) => Ok(<AnnotazioneService as RetrieveManyService<
            AnnotazioneEdificioDTO,
        >>::retrieve_many(db)
        .await
        .map_err(|e| e.to_string())?
        .into_iter()
        .map(AnnotazioneDTO::from)
        .collect::<Vec<AnnotazioneDTO>>()),
        TableWithPrimaryKey::Stanza(..) => Ok(<AnnotazioneService as RetrieveManyService<
            AnnotazioneStanzaDTO,
        >>::retrieve_many(db)
        .await
        .map_err(|e| e.to_string())?
        .into_iter()
        .map(AnnotazioneDTO::from)
        .collect::<Vec<AnnotazioneDTO>>()),
        TableWithPrimaryKey::Infisso(..) => Ok(<AnnotazioneService as RetrieveManyService<
            AnnotazioneInfissoDTO,
        >>::retrieve_many(db)
        .await
        .map_err(|e| e.to_string())?
        .into_iter()
        .map(AnnotazioneDTO::from)
        .collect::<Vec<AnnotazioneDTO>>()),
    }
}

#[tauri::command]
pub async fn insert_annotazione(
    db: State<'_, DatabaseManager>,
    annotazione: AnnotazioneDTO,
) -> ResultCommand<AnnotazioneDTO> {
    match annotazione.ref_table {
        TableWithPrimaryKey::Edificio(..) => Ok(<AnnotazioneService as CreateService<
            AnnotazioneEdificioDTO,
        >>::create(db, annotazione.into())
        .await
        .map_err(|e| e.to_string())?
        .into()),
        TableWithPrimaryKey::Stanza(..) => Ok(<AnnotazioneService as CreateService<
            AnnotazioneStanzaDTO,
        >>::create(db, annotazione.into())
        .await
        .map_err(|e| e.to_string())?
        .into()),
        TableWithPrimaryKey::Infisso(..) => Ok(<AnnotazioneService as CreateService<
            AnnotazioneInfissoDTO,
        >>::create(db, annotazione.into())
        .await
        .map_err(|e| e.to_string())?
        .into()),
    }
}

#[cfg(test)]
mod tests {
    use app_utils::{
        app_interface::{
            database_interface::DatabaseManagerTrait, service_interface::SelectedEdificioTrait,
        },
        test::impl_database_connector::IsolatedTestDatabaseConnector,
    };
    use tauri::{Listener, Manager};
    use tokio::sync::RwLock;

    use super::*;

    #[tokio::test]
    async fn test_add_new_fascicolo_from_xlsx() {
        let app = tauri::test::mock_app();
        let app_handle = app.handle();

        let db =
            DatabaseManager::with_connector(Box::new(IsolatedTestDatabaseConnector::new().await))
                .await;
        app.manage(db);
        let db_state = app.state::<DatabaseManager>();

        let edificio_selected = SelectedEdificioState::new(RwLock::new(EdificioSelected::new()));
        app.manage(edificio_selected);
        let selected_edificio = app.state::<SelectedEdificioState<EdificioSelected>>();

        let path = "/home/maty/Downloads/Telegram Desktop/scuole massalongo e coccinelle.xlsx";

        app.listen("edificio-changed", |event| {
            println!("Received event: {:?}", event);
            assert!(event.payload().contains("edificio_change"))
        });

        match add_new_fascicolo_from_xlsx(
            app_handle.clone(),
            db_state.clone(),
            selected_edificio.clone(),
            path.to_string(),
        )
        .await
        {
            Ok(_) => {}
            Err(e) => panic!("{e}"),
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let edificio = EdificioService::retrieve_many(db_state.clone())
            .await
            .unwrap();
        println!("{:?}", edificio);
        assert_eq!(edificio.len(), 1);
        assert_eq!(
            edificio[0].chiave,
            selected_edificio.read().await.get_chiave().unwrap()
        );

        let stanze =
            StanzaService::retrieve_by_edificio_selected(db_state, selected_edificio.clone())
                .await
                .ok()
                .unwrap();
        println!("{:?}", stanze);
        assert_eq!(stanze.len(), 68);
    }
}
