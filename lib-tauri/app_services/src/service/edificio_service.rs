use crate::dao::EdificioDAO;
use crate::dto::EdificioDTO;
use app_error::{AppResult, ApplicationError};
use app_interface::dao_interface::crud_operations::{Get, GetAll, Insert, Update};
use app_interface::database_interface::DatabaseManager;
use app_interface::service_interface::{
    CreateService, RetrieveManyService, RetrieveOneService, UpdateService,
};
use async_trait::async_trait;
use tauri::State;

pub struct EdificioService;

#[async_trait]
impl RetrieveManyService<EdificioDTO> for EdificioService {
    async fn retrieve_many(
        db: State<'_, impl DatabaseManager + Send + Sync>,
    ) -> Result<Vec<EdificioDTO>, ApplicationError> {
        let mut conn = db.get_connection().await?;
        let result = EdificioDAO::get_all(&mut conn)?;
        Ok(result.iter().map(EdificioDTO::from).collect())
    }
}

#[async_trait]
impl RetrieveOneService<EdificioDTO, String> for EdificioService {
    async fn retrieve_one(
        db: State<'_, impl DatabaseManager + Send + Sync>,
        id: String,
    ) -> AppResult<EdificioDTO> {
        let mut conn = db.get_connection().await?;
        let result = EdificioDAO::get(&mut conn, id)?;
        Ok(EdificioDTO::from(&result))
    }
}

#[async_trait]
impl CreateService<EdificioDTO> for EdificioService {
    async fn create(
        db: State<'_, impl DatabaseManager + Send + Sync>,
        item: EdificioDTO,
    ) -> AppResult<EdificioDTO> {
        let mut conn = db.get_connection().await?;
        let result = EdificioDAO::insert(&mut conn, item.into())?;
        Ok(EdificioDTO::from(&result))
    }
}

#[async_trait]
impl UpdateService<EdificioDTO> for EdificioService {
    async fn update(
        db: State<'_, impl DatabaseManager + Send + Sync>,
        edificio: EdificioDTO,
    ) -> Result<EdificioDTO, ApplicationError> {
        let mut conn = db.get_connection().await?;
        let result = EdificioDAO::update(&mut conn, edificio.chiave.clone(), edificio.into())?;
        Ok(EdificioDTO::from(&result))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_retrieve_many() {}
}
