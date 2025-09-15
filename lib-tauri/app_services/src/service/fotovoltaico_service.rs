use crate::dao::FotovoltaicoDAO;
use crate::dto::FotovoltaicoDTO;
use app_state::selected_edificio::SelectedEdificioTrait;
use app_utils::app_error::{AppResult, ApplicationError};
use app_utils::app_interface::dao_interface::crud_operations::{Get, GetAll, Insert};
use app_utils::app_interface::database_interface::DatabaseManagerTrait;
use app_utils::app_interface::service_interface::{
    CreateService, RetrieveByEdificioSelected, RetrieveManyService, SelectedEdificioState,
};
use async_trait::async_trait;
use std::ops::Deref;
use tauri::State;

pub struct FotovoltaicoService;

#[async_trait]
impl RetrieveManyService<FotovoltaicoDTO> for FotovoltaicoService {
    async fn retrieve_many(
        db: State<'_, impl DatabaseManagerTrait + Send + Sync>,
    ) -> AppResult<Vec<FotovoltaicoDTO>> {
        let mut conn = db.get_connection().await?;
        let utenze = FotovoltaicoDAO::get_all(&mut conn)?;
        Ok(utenze.iter().map(FotovoltaicoDTO::from).collect())
    }
}

#[async_trait]
impl RetrieveByEdificioSelected<FotovoltaicoDTO> for FotovoltaicoService {
    async fn retrieve_by_edificio_selected<S>(
        db_state: State<'_, impl DatabaseManagerTrait + Send + Sync>,
        edificio_selected_state: State<'_, SelectedEdificioState<S>>,
    ) -> AppResult<Vec<FotovoltaicoDTO>>
    where
        S: SelectedEdificioTrait + Send + Sync,
    {
        let mut conn = db_state.get_connection().await?;
        let edificio_selected = edificio_selected_state.read().await.deref().get_chiave();
        if edificio_selected.is_none() {
            return Err(ApplicationError::EdificioNotSelected);
        }

        let utenze = FotovoltaicoDAO::get(&mut conn, edificio_selected.unwrap())?;
        Ok(utenze.iter().map(FotovoltaicoDTO::from).collect())
    }
}

#[async_trait]
impl CreateService<FotovoltaicoDTO> for FotovoltaicoService {
    async fn create(
        db: State<'_, impl DatabaseManagerTrait + Send + Sync>,
        item: FotovoltaicoDTO,
    ) -> AppResult<FotovoltaicoDTO> {
        let mut conn = db.get_connection().await?;
        let utenza = FotovoltaicoDAO::insert(&mut conn, item.into())?;
        Ok(FotovoltaicoDTO::from(&utenza))
    }
}
