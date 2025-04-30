use crate::dao::crud_operations::{GetAll, Insert};
use crate::dao::entity::CommentoStanza;
use crate::dao::utils::schema_operations::CreateTable;
use crate::database::DatabaseConnection;

pub struct CommentoStanzaDAO;

impl CreateTable for CommentoStanzaDAO {
    fn create_table<C: DatabaseConnection>(conn: &C) -> Result<(), String> {
        todo!()
    }
}

impl GetAll<CommentoStanza> for CommentoStanzaDAO {
    fn get_all<C: DatabaseConnection>(conn: &C) -> Result<Vec<CommentoStanza>, String> {
        todo!()
    }
}

impl Insert<CommentoStanza> for CommentoStanzaDAO {
    fn insert<C: DatabaseConnection>(
        conn: &C,
        commento: CommentoStanza,
    ) -> Result<CommentoStanza, String> {
        todo!()
    }
}
