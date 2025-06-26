use crate::app_traits::{ConvertibleDto, FromEntity, GetAll, Insert, Update};
use crate::app_traits::{CreateService, RetrieveManyService, UpdateService};
use crate::dao::InfissoDAO;
use crate::db::Database;
use crate::dto::InfissoDTO;
use crate::utils::AppError;
use tauri::State;

pub struct InfissoService;

impl RetrieveManyService<InfissoDTO> for InfissoService {
    fn retrieve_many(db: State<'_, Database>) -> Result<Vec<InfissoDTO>, AppError> {
        let conn = db.get_conn()?;
        if let Some(conn) = conn.as_ref() {
            let result = InfissoDAO::get_all(conn)?;
            Ok(result
                .iter()
                .map(|e| InfissoDTO::from_entity(e.clone()))
                .collect())
        } else {
            Err(AppError::DatabaseNotInitialized)
        }
    }
}

impl CreateService<InfissoDTO> for InfissoService {
    fn create(db: State<'_, Database>, infisso: InfissoDTO) -> Result<InfissoDTO, AppError> {
        let conn = db.get_conn()?;
        if let Some(conn) = conn.as_ref() {
            let result = InfissoDAO::insert(conn, infisso.into_entity())?;
            Ok(InfissoDTO::from_entity(result))
        } else {
            Err(AppError::DatabaseNotInitialized)
        }
    }
}

impl UpdateService<InfissoDTO> for InfissoService {
    fn update(db: State<'_, Database>, infisso: InfissoDTO) -> Result<InfissoDTO, AppError> {
        let conn = db.get_conn()?;
        if let Some(conn) = conn.as_ref() {
            let result = InfissoDAO::update(conn, infisso.into_entity())?;
            Ok(InfissoDTO::from_entity(result))
        } else {
            Err(AppError::DatabaseNotInitialized)
        }
    }
}
