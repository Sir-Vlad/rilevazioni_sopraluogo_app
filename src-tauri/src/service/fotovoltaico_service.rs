use crate::app_traits::{GetAll, Insert};
use crate::dao::FotovoltaicoDAO;
use crate::database::Database;
use crate::dto::FotovoltaicoDTO;
use crate::service::utils::{CreateService, RetrieveManyService};
use crate::utils::AppError;
use tauri::State;

pub struct FotovoltaicoService;

impl RetrieveManyService<FotovoltaicoDTO> for FotovoltaicoService {
    fn retrieve_many(db: State<'_, Database>) -> Result<Vec<FotovoltaicoDTO>, AppError> {
        let conn = db.get_conn()?;
        if let Some(conn) = conn.as_ref() {
            let utenze = FotovoltaicoDAO::get_all(conn)?;
            Ok(utenze.iter().map(FotovoltaicoDTO::from).collect())
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
            let utenza = FotovoltaicoDAO::insert(conn, fotovoltaico.clone().into())?;
            Ok(FotovoltaicoDTO::from(&utenza))
        } else {
            Err(AppError::DatabaseNotInitialized)
        }
    }
}
