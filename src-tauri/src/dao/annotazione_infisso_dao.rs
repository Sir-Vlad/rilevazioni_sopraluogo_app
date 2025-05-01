use crate::dao::crud_operations::{GetAll, Insert};
use crate::dao::entity::AnnotazioneInfisso;
use crate::dao::utils::schema_operations::CreateTable;
use crate::dao::utils::DAO;
use crate::database::DatabaseConnection;

pub struct AnnotazioneInfissoDAO;

impl DAO for AnnotazioneInfissoDAO {
    fn table_name() -> &'static str {
        "ANNOTAZIONE_INFISSO"
    }
}

impl CreateTable for AnnotazioneInfissoDAO {
    fn create_table<C: DatabaseConnection>(conn: &C) -> Result<(), String> {
        todo!()
    }
}

impl GetAll<AnnotazioneInfisso> for AnnotazioneInfissoDAO {
    fn get_all<C: DatabaseConnection>(conn: &C) -> Result<Vec<AnnotazioneInfisso>, String> {
        todo!()
    }
}

impl Insert<AnnotazioneInfisso> for AnnotazioneInfissoDAO {
    fn insert<C: DatabaseConnection>(
        conn: &C,
        commento: AnnotazioneInfisso,
    ) -> Result<AnnotazioneInfisso, String> {
        todo!()
    }
}
