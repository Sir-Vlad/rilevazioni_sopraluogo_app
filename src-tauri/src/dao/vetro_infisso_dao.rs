use crate::dao::entities::entity::VetroInfisso;
use crate::database::{QueryBuilder, SqlQueryBuilder};
use rusqlite::Connection;

pub trait VetroInfissoDAO {
    fn get_all(conn: &Connection) -> Result<Vec<VetroInfisso>, String>;
}

pub struct VetroInfissoDAOImpl;

impl VetroInfissoDAO for VetroInfissoDAOImpl {
    fn get_all(conn: &Connection) -> Result<Vec<VetroInfisso>, String> {
        let (query, _) = QueryBuilder::select()
            .table("VETRO_INFISSO")
            .build()
            .map_err(|e| e.to_string())?;

        let mut stmt = conn
            .prepare(query.as_str())
            .map_err(|e| format!("Errore nella creazione della query: {}", e))?;

        let result: Result<Vec<VetroInfisso>, rusqlite::Error> = stmt
            .query_map([], |row| {
                Ok(VetroInfisso {
                    id: row.get::<_, u64>("ID")?,
                    vetro: row.get::<_, String>("VETRO")?,
                    efficienza_energetica: row.get::<_, u8>("EFFICIENZA_ENERGETICA")?,
                })
            })
            .expect("Errore nella lettura dei dati di tipo materiale")
            .collect();
        result.map_err(|e| e.to_string())
    }
}
