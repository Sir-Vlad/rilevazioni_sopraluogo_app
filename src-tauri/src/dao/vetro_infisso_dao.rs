use crate::dao::entity::VetroInfisso;
use rusqlite::Connection;

pub trait VetroInfissoDao {
    fn get_all(conn: &Connection) -> Result<Vec<VetroInfisso>, String>;
}

pub struct VetroInfissoDaoImpl;

impl VetroInfissoDao for VetroInfissoDaoImpl {
    fn get_all(conn: &Connection) -> Result<Vec<VetroInfisso>, String> {
        let mut stmt = conn
            .prepare("SELECT * FROM VETRO_INFISSO")
            .map_err(|e| format!("Errore nella creazione della query: {}", e))?;

        let result: Result<Vec<VetroInfisso>, rusqlite::Error> = stmt
            .query_map([], |row| {
                Ok(VetroInfisso {
                    id: row.get::<_, u64>(0)?,
                    vetro: row.get::<_, String>(1)?,
                    efficienza_energetica: row.get::<_, i8>(2)?,
                })
            })
            .expect("Errore nella lettura dei dati di tipo materiale")
            .collect();
        result.map_err(|e| e.to_string())
    }
}
