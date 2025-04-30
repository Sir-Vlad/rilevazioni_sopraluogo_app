use crate::dao::crud_operations::{GetAll, Insert};
use crate::dao::entity::CommentoEdificio;
use crate::dao::utils::schema_operations::CreateTable;
use crate::database::DatabaseConnection;

pub struct CommentoEdificioDAO;

impl CreateTable for CommentoEdificioDAO {
    fn create_table<C: DatabaseConnection>(conn: &C) -> Result<(), String> {
        todo!()
    }
}

impl GetAll<CommentoEdificio> for CommentoEdificioDAO {
    fn get_all<C: DatabaseConnection>(conn: &C) -> Result<Vec<CommentoEdificio>, String> {
        todo!()
    }
}

impl Insert<CommentoEdificio> for CommentoEdificioDAO {
    fn insert<C: DatabaseConnection>(
        conn: &C,
        commento: CommentoEdificio,
    ) -> Result<CommentoEdificio, String> {
        todo!()
    }
}
