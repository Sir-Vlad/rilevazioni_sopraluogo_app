use crate::events_payload::EdificioChangeEventPayload;
use crate::{get_chiave_selected_edificio, is_selected_edificio};
use app_services::dto::TableWithPrimaryKey;
use app_services::{
    dto::{
        AnnotazioneDTO, AnnotazioneEdificioDTO, AnnotazioneInfissoDTO, AnnotazioneStanzaDTO,
        EdificioDTO, FotovoltaicoDTO, InfissoDTO, StanzaDTO, TipoDTO, UtenzaDTO,
    },
    service::{
        AnnotazioneService, CreateService, EdificioService, FotovoltaicoService, InfissoService,
        StanzaService, TypeService, TypeServiceImpl, UpdateService, UtenzeService,
    },
};
use app_state::database::DatabaseManager;
use app_state::selected_edificio::StateEdificioSelected;
use app_utils::app_interface::service_interface::RetrieveManyService;
use log::info;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use tauri::{AppHandle, Emitter, State};

type ResultCommand<T> = Result<T, String>;

/**************************************************************************************************/
/******************************* COMMAND PER MISCELLANEOUS **********************************/
/**************************************************************************************************/

#[tauri::command]
pub async fn get_fascicoli(db: State<'_, DatabaseManager>) -> ResultCommand<Vec<i32>> {
    let edifici = EdificioService::retrieve_many(db).await;
    match edifici {
        Ok(edifici) => {
            let unique_fascicoli: HashSet<i32> = edifici
                .iter()
                .map(|edificio| edificio.fascicolo)
                .collect();
            let mut res = unique_fascicoli.into_iter().collect::<Vec<i32>>();
            res.sort();
            Ok(res)
        }
        Err(_) => Ok(Vec::new()),
    }
}

/**************************************************************************************************/
/******************************* COMMAND PER GESTIRE IL DATABASE **********************************/
/**************************************************************************************************/

#[tauri::command]
pub async fn set_edificio(
    app_handle: AppHandle,
    edificio_selected: State<'_, StateEdificioSelected>,
    chiave: String,
) -> ResultCommand<()> {
    info!("Switching edificio to {}", chiave);
    EdificioService::select_edificio(edificio_selected.clone(), chiave).await;

    app_handle
        .emit(
            "edificio-changed",
            EdificioChangeEventPayload {
                type_event: "edificio_change",
                chiave: get_chiave_selected_edificio(edificio_selected).await.unwrap(),
            },
        )
        .map_err(|e| e.to_string())?;

    info!("Edificio switched");
    Ok(())
}

#[tauri::command]
pub async fn clear_edificio(edificio_selected: State<'_, StateEdificioSelected>) -> ResultCommand<()> {
    EdificioService::clear_edificio(edificio_selected).await;
    info!("Edificio cleared");
    Ok(())
}

/**************************************************************************************************/
/***************************** COMMAND PER INIZIALIZZARE IL SISTEMA *******************************/
/**************************************************************************************************/

/*
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
pub async fn insert_tipo(db: State<'_, DatabaseManager>, tipo: TipoDTO) -> ResultCommand<TipoDTO> {
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
pub async fn get_utenze(db: State<'_, DatabaseManager>, selected_edificio: State<'_, StateEdificioSelected>) -> ResultCommand<Vec<UtenzaDTO>> {
    if !is_selected_edificio(selected_edificio.clone()).await {
        return Ok(Vec::new());
    }

    UtenzeService::retrieve_many(db)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn insert_utenza(
    db: State<'_, DatabaseManager>,
    selected_edificio: State<'_, StateEdificioSelected>,
    utenza: UtenzaDTO,
) -> ResultCommand<UtenzaDTO> {
    if !is_selected_edificio(selected_edificio.clone()).await {
        return Err("Non selezionato un edificio".to_string());
    }

    if get_chiave_selected_edificio(selected_edificio).await.unwrap() != utenza.edificio_id {
        return Err("Chiave non corrispondente".to_string());
    }

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
    selected_edificio: State<'_, StateEdificioSelected>,
) -> ResultCommand<Vec<FotovoltaicoDTO>> {
    if !is_selected_edificio(selected_edificio.clone()).await {
        return Ok(Vec::new());
    }

    FotovoltaicoService::retrieve_many(db)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn insert_fotovoltaico(
    db: State<'_, DatabaseManager>,
    selected_edificio: State<'_, StateEdificioSelected>,
    fotovoltaico: FotovoltaicoDTO,
) -> ResultCommand<FotovoltaicoDTO> {
    if !is_selected_edificio(selected_edificio.clone()).await {
        return Err("Non selezionato un edificio".to_string());
    }

    if get_chiave_selected_edificio(selected_edificio).await.unwrap() != fotovoltaico.id_edificio {
        return Err("Chiave non corrispondente".to_string());
    }

    FotovoltaicoService::create(db, fotovoltaico)
        .await
        .map_err(|e| e.to_string())
}

/**************************************************************************************************/
/********************************** COMMAND PER ANNOTAZIONI ***************************************/
/**************************************************************************************************/
#[tauri::command]
pub async fn get_annotazioni(
    db: State<'_, DatabaseManager>,
    table: TableWithPrimaryKey,
) -> ResultCommand<Vec<AnnotazioneDTO>> {
    match table {
        TableWithPrimaryKey::Edificio(..) => Ok(
            <AnnotazioneService as RetrieveManyService<AnnotazioneEdificioDTO>>::retrieve_many(db)
                .await
                .map_err(|e| e.to_string())?
                .into_iter()
                .map(AnnotazioneDTO::from)
                .collect::<Vec<AnnotazioneDTO>>(),
        ),
        TableWithPrimaryKey::Stanza(..) => Ok(
            <AnnotazioneService as RetrieveManyService<AnnotazioneStanzaDTO>>::retrieve_many(db)
                .await
                .map_err(|e| e.to_string())?
                .into_iter()
                .map(AnnotazioneDTO::from)
                .collect::<Vec<AnnotazioneDTO>>(),
        ),
        TableWithPrimaryKey::Infisso(..) => Ok(
            <AnnotazioneService as RetrieveManyService<AnnotazioneInfissoDTO>>::retrieve_many(db)
                .await
                .map_err(|e| e.to_string())?
                .into_iter()
                .map(AnnotazioneDTO::from)
                .collect::<Vec<AnnotazioneDTO>>(),
        )
    }
}

#[tauri::command]
pub async fn insert_annotazione(
    db: State<'_, DatabaseManager>,
    annotazione: AnnotazioneDTO,
) -> ResultCommand<AnnotazioneDTO> {
    match annotazione.ref_table {
        TableWithPrimaryKey::Edificio(..) => Ok(
            <AnnotazioneService as CreateService<AnnotazioneEdificioDTO>>::create(
                db,
                annotazione.into(),
            )
                .await
                .map_err(|e| e.to_string())?
                .into(),
        ),
        TableWithPrimaryKey::Stanza(..) => Ok(
            <AnnotazioneService as CreateService<AnnotazioneStanzaDTO>>::create(
                db,
                annotazione.into(),
            )
                .await
                .map_err(|e| e.to_string())?
                .into(),
        ),
        TableWithPrimaryKey::Infisso(..) => Ok(
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
            table = annotazione.ref_table
        )),
    }
}
