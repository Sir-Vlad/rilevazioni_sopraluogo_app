use crate::app_traits::{ConvertibleDto, FromEntity, GetAll, Insert};
use crate::app_traits::{CreateService, RetrieveManyService};
use crate::dao::FotovoltaicoDAO;
use crate::db::Database;
use crate::dto::FotovoltaicoDTO;
use crate::utils::AppError;
use tauri::State;

pub struct FotovoltaicoService;

impl RetrieveManyService<FotovoltaicoDTO> for FotovoltaicoService {
    fn retrieve_many(db: State<'_, Database>) -> Result<Vec<FotovoltaicoDTO>, AppError> {
        let conn = db.get_conn()?;
        if let Some(conn) = conn.as_ref() {
            let utenze = FotovoltaicoDAO::get_all(conn)?;
            Ok(utenze
                .iter()
                .map(|e| FotovoltaicoDTO::from_entity(e.clone()))
                .collect())
        } else {
            Err(AppError::DatabaseNotInitialized)
        }
    }
}

impl CreateService<FotovoltaicoDTO> for FotovoltaicoService {
    fn create(
        db: State<'_, Database>,
        fotovoltaico: FotovoltaicoDTO,
    ) -> Result<FotovoltaicoDTO, AppError> {
        let conn = db.get_conn()?;
        if let Some(conn) = conn.as_ref() {
            let utenza = FotovoltaicoDAO::insert(conn, fotovoltaico.into_entity())?;
            Ok(FotovoltaicoDTO::from_entity(utenza))
        } else {
            Err(AppError::DatabaseNotInitialized)
        }
    }
}
