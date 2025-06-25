use crate::app_traits::{GetAll, Insert};
use crate::dao::UtenzeDAO;
use crate::db::Database;
use crate::dto::UtenzaDTO;
use crate::service::utils::{CreateService, RetrieveManyService};
use crate::utils::AppError;
use tauri::State;

pub struct UtenzeService;

impl RetrieveManyService<UtenzaDTO> for UtenzeService {
    fn retrieve_many(db: State<'_, Database>) -> Result<Vec<UtenzaDTO>, AppError> {
        let conn = db.get_conn()?;
        if let Some(conn) = conn.as_ref() {
            let utenze = UtenzeDAO::get_all(conn)?;
            Ok(utenze.iter().map(UtenzaDTO::from).collect())
        } else {
            Err(AppError::DatabaseNotInitialized)
        }
    }
}

impl CreateService<UtenzaDTO> for UtenzeService {
    fn create(db: State<'_, Database>, utenza: UtenzaDTO) -> Result<UtenzaDTO, AppError> {
        let conn = db.get_conn()?;
        if let Some(conn) = conn.as_ref() {
            let utenza = UtenzeDAO::insert(conn, utenza.clone().into())?;
            Ok(UtenzaDTO::from(&utenza))
        } else {
            Err(AppError::DatabaseNotInitialized)
        }
    }
}
