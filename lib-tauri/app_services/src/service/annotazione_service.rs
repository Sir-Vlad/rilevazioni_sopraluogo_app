use crate::dao::{AnnotazioneEdificioDAO, AnnotazioneInfissoDAO, AnnotazioneStanzaDAO};
use crate::dto::{AnnotazioneEdificioDTO, AnnotazioneInfissoDTO, AnnotazioneStanzaDTO};
use app_utils::app_error::AppResult;
use app_utils::app_interface::dao_interface::crud_operations::{GetAll, Insert};
use app_utils::app_interface::database_interface::DatabaseManager;
use app_utils::app_interface::service_interface::{CreateService, RetrieveManyService};
use async_trait::async_trait;
use tauri::State;

pub struct AnnotazioneService;

#[async_trait]
impl RetrieveManyService<AnnotazioneEdificioDTO> for AnnotazioneService {
    async fn retrieve_many(
        db: State<'_, impl DatabaseManager + Send + Sync>,
    ) -> AppResult<Vec<AnnotazioneEdificioDTO>> {
        let mut conn = db.get_connection().await?;
        let result = AnnotazioneEdificioDAO::get_all(&mut conn)?;
        Ok(result
            .into_iter()
            .map(AnnotazioneEdificioDTO::from)
            .collect())
    }
}

#[async_trait]
impl CreateService<AnnotazioneEdificioDTO> for AnnotazioneService {
    async fn create(
        db: State<'_, impl DatabaseManager + Send + Sync>,
        item: AnnotazioneEdificioDTO,
    ) -> AppResult<AnnotazioneEdificioDTO> {
        let mut conn = db.get_connection().await?;
        let result = AnnotazioneEdificioDAO::insert(&mut conn, item.into())?;
        Ok(AnnotazioneEdificioDTO::from(result))
    }
}

#[async_trait]
impl RetrieveManyService<AnnotazioneStanzaDTO> for AnnotazioneService {
    async fn retrieve_many(
        db: State<'_, impl DatabaseManager + Send + Sync>,
    ) -> AppResult<Vec<AnnotazioneStanzaDTO>> {
        let mut conn = db.get_connection().await?;
        let result = AnnotazioneStanzaDAO::get_all(&mut conn)?;
        Ok(result.into_iter().map(AnnotazioneStanzaDTO::from).collect())
    }
}

#[async_trait]
impl CreateService<AnnotazioneStanzaDTO> for AnnotazioneService {
    async fn create(
        db: State<'_, impl DatabaseManager + Send + Sync>,
        item: AnnotazioneStanzaDTO,
    ) -> AppResult<AnnotazioneStanzaDTO> {
        let mut conn = db.get_connection().await?;
        let result = AnnotazioneStanzaDAO::insert(&mut conn, item.into())?;
        Ok(AnnotazioneStanzaDTO::from(result))
    }
}

#[async_trait]
impl RetrieveManyService<AnnotazioneInfissoDTO> for AnnotazioneService {
    async fn retrieve_many(
        db: State<'_, impl DatabaseManager + Send + Sync>,
    ) -> AppResult<Vec<AnnotazioneInfissoDTO>> {
        let mut conn = db.get_connection().await?;
        let result = AnnotazioneInfissoDAO::get_all(&mut conn)?;
        Ok(result
            .into_iter()
            .map(AnnotazioneInfissoDTO::from)
            .collect())
    }
}

#[async_trait]
impl CreateService<AnnotazioneInfissoDTO> for AnnotazioneService {
    async fn create(
        db: State<'_, impl DatabaseManager + Send + Sync>,
        item: AnnotazioneInfissoDTO,
    ) -> AppResult<AnnotazioneInfissoDTO> {
        let mut conn = db.get_connection().await?;
        let result = AnnotazioneInfissoDAO::insert(&mut conn, item.into())?;
        Ok(AnnotazioneInfissoDTO::from(result))
    }
}
