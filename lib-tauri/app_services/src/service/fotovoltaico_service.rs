use crate::dao::FotovoltaicoDAO;
use crate::dto::FotovoltaicoDTO;
use app_state::selected_edificio::SelectedEdificioTrait;
use app_utils::app_error::{AppResult, ApplicationError, DomainError, ErrorKind};
use app_utils::app_interface::dao_interface::crud_operations::{Get, GetAll, Insert};
use app_utils::app_interface::database_interface::DatabaseManagerTrait;
use app_utils::app_interface::service_interface::{
    CreateService, RetrieveBy, RetrieveByEdificioSelected, RetrieveManyService,
    SelectedEdificioState,
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
impl RetrieveBy<FotovoltaicoDTO> for FotovoltaicoService {
    type Output = Vec<FotovoltaicoDTO>;

    async fn retrieve_by(
        db_state: State<'_, impl DatabaseManagerTrait + Send + Sync>,
        where_field: &str,
        where_value: &str,
    ) -> AppResult<Self::Output> {
        let mut conn = db_state.get_connection().await?;

        let result = match where_field {
            "edificio" => FotovoltaicoDAO::get(&mut conn, where_value.to_string())?,
            _ => {
                return Err(DomainError::InvalidInput(
                    ErrorKind::InvalidField,
                    where_field.to_string(),
                )
                    .into())
            }
        };

        Ok(result.iter().map(FotovoltaicoDTO::from).collect())
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
        let edificio_selected = edificio_selected_state.read().await.deref().get_chiave();
        if edificio_selected.is_none() {
            return Err(ApplicationError::EdificioNotSelected);
        }

        Self::retrieve_by(db_state, "edificio", edificio_selected.unwrap().deref()).await
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
