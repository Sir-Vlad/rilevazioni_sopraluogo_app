use crate::dao::crud_operations::{GetAll, Insert};
use crate::dao::{AnnotazioneEdificioDAO, AnnotazioneInfissoDAO, AnnotazioneStanzaDAO};
use crate::database::Database;
use crate::dto::{AnnotazioneEdificioDTO, AnnotazioneInfissoDTO, AnnotazioneStanzaDTO};
use crate::service::{CreateService, RetrieveManyService};
use crate::utils::AppError;
use tauri::State;

pub struct AnnotazioneService;

impl RetrieveManyService<AnnotazioneEdificioDTO> for AnnotazioneService {
    fn retrieve_many(db: State<'_, Database>) -> Result<Vec<AnnotazioneEdificioDTO>, AppError> {
        let conn = db.get_conn();
        if let Some(conn) = conn.as_ref() {
            let result = AnnotazioneEdificioDAO::get_all(conn)?;
            Ok(result
                .into_iter()
                .map(AnnotazioneEdificioDTO::from)
                .collect())
        } else {
            Err(AppError::DatabaseNotInitialized)
        }
    }
}

impl CreateService<AnnotazioneEdificioDTO> for AnnotazioneService {
    fn create(
        db: State<'_, Database>,
        item: AnnotazioneEdificioDTO,
    ) -> Result<AnnotazioneEdificioDTO, AppError> {
        let conn = db.get_conn();
        if let Some(conn) = conn.as_ref() {
            let result = AnnotazioneEdificioDAO::insert(conn, item.clone().into())?;
            Ok(AnnotazioneEdificioDTO::from(result))
        } else {
            Err(AppError::DatabaseNotInitialized)
        }
    }
}

impl RetrieveManyService<AnnotazioneStanzaDTO> for AnnotazioneService {
    fn retrieve_many(db: State<'_, Database>) -> Result<Vec<AnnotazioneStanzaDTO>, AppError> {
        let conn = db.get_conn();
        if let Some(conn) = conn.as_ref() {
            let result = AnnotazioneStanzaDAO::get_all(conn)?;
            Ok(result.into_iter().map(AnnotazioneStanzaDTO::from).collect())
        } else {
            Err(AppError::DatabaseNotInitialized)
        }
    }
}

impl CreateService<AnnotazioneStanzaDTO> for AnnotazioneService {
    fn create(
        db: State<'_, Database>,
        item: AnnotazioneStanzaDTO,
    ) -> Result<AnnotazioneStanzaDTO, AppError> {
        let conn = db.get_conn();
        if let Some(conn) = conn.as_ref() {
            let result = AnnotazioneStanzaDAO::insert(conn, item.clone().into())?;
            Ok(AnnotazioneStanzaDTO::from(result))
        } else {
            Err(AppError::DatabaseNotInitialized)
        }
    }
}

impl RetrieveManyService<AnnotazioneInfissoDTO> for AnnotazioneService {
    fn retrieve_many(db: State<'_, Database>) -> Result<Vec<AnnotazioneInfissoDTO>, AppError> {
        let conn = db.get_conn();
        if let Some(conn) = conn.as_ref() {
            let result = AnnotazioneInfissoDAO::get_all(conn)?;
            Ok(result
                .into_iter()
                .map(AnnotazioneInfissoDTO::from)
                .collect())
        } else {
            Err(AppError::DatabaseNotInitialized)
        }
    }
}

impl CreateService<AnnotazioneInfissoDTO> for AnnotazioneService {
    fn create(
        db: State<'_, Database>,
        item: AnnotazioneInfissoDTO,
    ) -> Result<AnnotazioneInfissoDTO, AppError> {
        let conn = db.get_conn();
        if let Some(conn) = conn.as_ref() {
            let result = AnnotazioneInfissoDAO::insert(conn, item.clone().into())?;
            Ok(AnnotazioneInfissoDTO::from(result))
        } else {
            Err(AppError::DatabaseNotInitialized)
        }
    }
}
