use crate::dao::crud_operations::{GetAll, Insert, Update};
use crate::dao::InfissoDAO;
use crate::database::Database;
use crate::dto::InfissoDTO;
use crate::service::utils::{CreateService, RetrieveManyService, UpdateService};
use crate::utils::AppError;
use tauri::State;

pub struct InfissoService;

impl RetrieveManyService<InfissoDTO> for InfissoService {
    fn retrieve_many(db: State<'_, Database>) -> Result<Vec<InfissoDTO>, AppError> {
        let conn = db.get_conn();
        if let Some(conn) = conn.as_ref() {
            let result = InfissoDAO::get_all(conn)?;
            Ok(result.iter().map(InfissoDTO::from).collect())
        } else {
            Err(AppError::DatabaseNotInitialized)
        }
    }
}

impl CreateService<InfissoDTO> for InfissoService {
    fn create(db: State<'_, Database>, infisso: InfissoDTO) -> Result<InfissoDTO, AppError> {
        let conn = db.get_conn();
        if let Some(conn) = conn.as_ref() {
            let result = InfissoDAO::insert(conn, infisso.clone().into())?;
            Ok(InfissoDTO::from(&result))
        } else {
            Err(AppError::DatabaseNotInitialized)
        }
    }
}

impl UpdateService<InfissoDTO> for InfissoService {
    fn update(db: State<'_, Database>, infisso: InfissoDTO) -> Result<InfissoDTO, AppError> {
        let conn = db.get_conn();
        if let Some(conn) = conn.as_ref() {
            let result = InfissoDAO::update(conn, infisso.clone().into())?;
            Ok(InfissoDTO::from(&result))
        } else {
            Err(AppError::DatabaseNotInitialized)
        }
    }
}
