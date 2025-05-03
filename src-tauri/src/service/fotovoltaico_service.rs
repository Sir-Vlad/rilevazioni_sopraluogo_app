use crate::database::Database;
use crate::dto::FotovoltaicoDTO;
use crate::service::utils::{CreateService, RetrieveManyService};
use crate::utils::AppError;
use tauri::State;

pub struct FotovoltaicoService;

impl RetrieveManyService<FotovoltaicoDTO> for FotovoltaicoService {
    fn retrieve_many(db: State<'_, Database>) -> Result<Vec<FotovoltaicoDTO>, AppError> {
        todo!()
    }
}

impl CreateService<FotovoltaicoDTO> for FotovoltaicoService {
    fn create(
        db: State<'_, Database>,
        utenza: FotovoltaicoDTO,
    ) -> Result<FotovoltaicoDTO, AppError> {
        todo!()
    }
}
