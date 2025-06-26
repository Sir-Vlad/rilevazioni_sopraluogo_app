use crate::app_traits::{ConvertibleDto, FromEntity, GetAll, Update};
use crate::app_traits::{RetrieveManyService, UpdateService};
use crate::dao::EdificioDAO;
use crate::db::Database;
use crate::dto::EdificioDTO;
use crate::utils::AppError;
use tauri::State;

pub struct EdificioService;

impl RetrieveManyService<EdificioDTO> for EdificioService {
    fn retrieve_many(db: State<'_, Database>) -> Result<Vec<EdificioDTO>, AppError> {
        let conn = db.get_conn()?;
        if let Some(conn) = conn.as_ref() {
            let result = EdificioDAO::get_all(conn)?;
            Ok(result
                .iter()
                .map(|e| EdificioDTO::from_entity(e.clone()))
                .collect())
        } else {
            Err(AppError::DatabaseNotInitialized)
        }
    }
}

impl UpdateService<EdificioDTO> for EdificioService {
    fn update(db: State<'_, Database>, edificio: EdificioDTO) -> Result<EdificioDTO, AppError> {
        let conn = db.get_conn()?;
        if let Some(conn) = conn.as_ref() {
            let result = EdificioDAO::update(conn, edificio.into_entity())?;
            Ok(EdificioDTO::from_entity(result))
        } else {
            Err(AppError::DatabaseNotInitialized)
        }
    }
}
