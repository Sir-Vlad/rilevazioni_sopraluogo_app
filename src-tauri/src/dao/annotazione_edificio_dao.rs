use crate::dao::crud_operations::{GetAll, Insert};
use crate::dao::entity::AnnotazioneEdificio;
use crate::dao::utils::schema_operations::CreateTable;
use crate::dao::utils::DAO;
use crate::database::DatabaseConnection;
use crate::utils::AppError;

pub struct AnnotazioneEdificioDAO;

impl DAO for AnnotazioneEdificioDAO {
    fn table_name() -> &'static str {
        "ANNOTAZIONE_EDIFICIO"
    }
}

impl CreateTable for AnnotazioneEdificioDAO {
    fn create_table<C: DatabaseConnection>(conn: &C) -> Result<(), AppError> {
        todo!()
    }
}

impl GetAll<AnnotazioneEdificio> for AnnotazioneEdificioDAO {
    fn get_all<C: DatabaseConnection>(conn: &C) -> Result<Vec<AnnotazioneEdificio>, AppError> {
        todo!()
    }
}

impl Insert<AnnotazioneEdificio> for AnnotazioneEdificioDAO {
    fn insert<C: DatabaseConnection>(
        conn: &C,
        item: AnnotazioneEdificio,
    ) -> Result<AnnotazioneEdificio, AppError> {
        todo!()
    }
}
