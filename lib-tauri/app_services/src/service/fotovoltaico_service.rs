use crate::dao::FotovoltaicoDAO;
use crate::dto::FotovoltaicoDTO;
use app_error::AppResult;
use app_interface::dao_interface::crud_operations::{GetAll, Insert};
use app_interface::database_interface::DatabaseManager;
use app_interface::service_interface::{CreateService, RetrieveManyService};
use async_trait::async_trait;
use tauri::State;

pub struct FotovoltaicoService;

#[async_trait]
impl RetrieveManyService<FotovoltaicoDTO> for FotovoltaicoService {
    async fn retrieve_many(
        db: State<'_, impl DatabaseManager + Send + Sync>,
    ) -> AppResult<Vec<FotovoltaicoDTO>> {
        let mut conn = db.get_connection().await?;
        let utenze = FotovoltaicoDAO::get_all(&mut conn)?;
        Ok(utenze.iter().map(FotovoltaicoDTO::from).collect())
    }
}

#[async_trait]
impl CreateService<FotovoltaicoDTO> for FotovoltaicoService {
    async fn create(
        db: State<'_, impl DatabaseManager + Send + Sync>,
        item: FotovoltaicoDTO,
    ) -> AppResult<FotovoltaicoDTO> {
        let mut conn = db.get_connection().await?;
        let utenza = FotovoltaicoDAO::insert(&mut conn, item.into())?;
        Ok(FotovoltaicoDTO::from(&utenza))
    }
}
