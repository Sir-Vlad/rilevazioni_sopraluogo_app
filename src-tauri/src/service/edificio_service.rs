use crate::dao::crud_operations::{GetAll, Update};
use crate::dao::EdificioDAO;
use crate::database::Database;
use crate::dto::EdificioDTO;
use crate::service::utils::{RetrieveManyService, UpdateService};
use crate::utils::AppError;
use tauri::State;

pub struct EdificioService;

impl RetrieveManyService<EdificioDTO> for EdificioService {
    fn retrieve_many(db: State<'_, Database>) -> Result<Vec<EdificioDTO>, AppError> {
        let conn = db.get_conn();
        if let Some(conn) = conn.as_ref() {
            let result = EdificioDAO::get_all(conn)?;
            Ok(result.iter().map(EdificioDTO::from).collect())
        } else {
            Err(AppError::DatabaseNotInitialized)
        }
    }
}

impl UpdateService<EdificioDTO> for EdificioService {
    fn update(db: State<'_, Database>, edificio: EdificioDTO) -> Result<EdificioDTO, AppError> {
        let conn = db.get_conn();
        if let Some(conn) = conn.as_ref() {
            let result = EdificioDAO::update(conn, edificio.into())?;
            Ok(EdificioDTO::from(&result))
        } else {
            Err(AppError::DatabaseNotInitialized)
        }
    }
}
