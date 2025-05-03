use crate::database::Database;
use crate::dto::UtenzaDTO;
use crate::service::utils::{CreateService, RetrieveManyService};
use crate::utils::AppError;
use tauri::State;

pub struct UtenzeService;

impl RetrieveManyService<UtenzaDTO> for UtenzeService {
    fn retrieve_many(db: State<'_, Database>) -> Result<Vec<UtenzaDTO>, AppError> {
        todo!()
    }
}

impl CreateService<UtenzaDTO> for UtenzeService {
    fn create(db: State<'_, Database>, utenza: UtenzaDTO) -> Result<UtenzaDTO, AppError> {
        todo!()
    }
}
