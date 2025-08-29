use crate::dao::InfissoDAO;
use crate::dto::InfissoDTO;
use app_utils::app_error::AppResult;
use app_utils::app_interface::dao_interface::crud_operations::{GetAll, Insert, Update};
use app_utils::app_interface::database_interface::DatabaseManager;
use app_utils::app_interface::service_interface::{CreateService, RetrieveManyService, UpdateService};
use async_trait::async_trait;
use tauri::State;

pub struct InfissoService;

#[async_trait]
impl RetrieveManyService<InfissoDTO> for InfissoService {
    async fn retrieve_many(
        db: State<'_, impl DatabaseManager + Send + Sync>,
    ) -> AppResult<Vec<InfissoDTO>> {
        let mut conn = db.get_connection().await?;
        let result = InfissoDAO::get_all(&mut conn)?;
        Ok(result.iter().map(InfissoDTO::from).collect())
    }
}

#[async_trait]
impl CreateService<InfissoDTO> for InfissoService {
    async fn create(
        db: State<'_, impl DatabaseManager + Send + Sync>,
        item: InfissoDTO,
    ) -> AppResult<InfissoDTO> {
        let mut conn = db.get_connection().await?;
        let result = InfissoDAO::insert(&mut conn, item.into())?;
        Ok(InfissoDTO::from(&result))
    }
}

#[async_trait]
impl UpdateService<InfissoDTO> for InfissoService {
    async fn update(
        db: State<'_, impl DatabaseManager + Send + Sync>,
        item: InfissoDTO,
    ) -> AppResult<InfissoDTO> {
        let mut conn = db.get_connection().await?;
        let result = InfissoDAO::update(
            &mut conn,
            (item.id.clone(), item.id_edificio.clone()),
            item.into(),
        )?;
        Ok(InfissoDTO::from(&result))
    }
}
