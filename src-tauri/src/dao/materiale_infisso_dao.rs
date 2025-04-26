use crate::dao::entities::entity::MaterialeInfisso;
use crate::database::{QueryBuilder, SqlQueryBuilder};
use rusqlite::Connection;

pub trait MaterialeInfissoDAO {
    fn get_all(conn: &Connection) -> Result<Vec<MaterialeInfisso>, String>;
}

pub struct MaterialeInfissoDAOImpl;

impl MaterialeInfissoDAO for MaterialeInfissoDAOImpl {
    fn get_all(conn: &Connection) -> Result<Vec<MaterialeInfisso>, String> {
        let (query, _) = QueryBuilder::select()
            .table("MATERIALE_INFISSO")
            .build()
            .map_err(|e| e.to_string())?;

        let mut stmt = conn
            .prepare(query.as_str())
            .map_err(|e| format!("Errore nella creazione della query: {}", e))?;

        let result: Result<Vec<MaterialeInfisso>, rusqlite::Error> = stmt
            .query_map([], |row| {
                Ok(MaterialeInfisso {
                    id: row.get::<_, u64>("ID")?,
                    materiale: row.get::<_, String>("MATERIALE")?,
                    efficienza_energetica: row.get::<_, u8>("EFFICIENZA_ENERGETICA")?,
                })
            })
            .expect("Errore nella lettura dei dati di tipo materiale")
            .collect();
        result.map_err(|e| e.to_string())
    }
}
