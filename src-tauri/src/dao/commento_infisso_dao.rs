use crate::dao::crud_operations::{GetAll, Insert};
use crate::dao::entity::CommentoInfisso;
use crate::dao::utils::schema_operations::CreateTable;
use crate::database::DatabaseConnection;

pub struct CommentoInfissoDAO;

impl GetAll<CommentoInfisso> for CommentoInfissoDAO {
    fn get_all<C: DatabaseConnection>(conn: &C) -> Result<Vec<CommentoInfisso>, String> {
        todo!()
    }
}

impl Insert<CommentoInfisso> for CommentoInfissoDAO {
    fn insert<C: DatabaseConnection>(
        conn: &C,
        commento: CommentoInfisso,
    ) -> Result<CommentoInfisso, String> {
        todo!()
    }
}

impl CreateTable for CommentoInfissoDAO {
    fn create_table<C: DatabaseConnection>(conn: &C) -> Result<(), String> {
        todo!()
    }
}
