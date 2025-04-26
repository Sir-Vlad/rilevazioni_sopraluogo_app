use crate::dao::entities::entity::VetroInfisso;
use rusqlite::Connection;

pub trait VetroInfissoDAO {
    fn get_all(conn: &Connection) -> Result<Vec<VetroInfisso>, String>;
}

pub struct VetroInfissoDAOImpl;

impl VetroInfissoDAO for VetroInfissoDAOImpl {
    fn get_all(conn: &Connection) -> Result<Vec<VetroInfisso>, String> {
        let mut stmt = conn
            .prepare("SELECT * FROM VETRO_INFISSO")
            .map_err(|e| format!("Errore nella creazione della query: {}", e))?;

        let result: Result<Vec<VetroInfisso>, rusqlite::Error> = stmt
            .query_map([], |row| {
                Ok(VetroInfisso {
                    id: row.get::<_, u64>(0)?,
                    vetro: row.get::<_, String>(1)?,
                    efficienza_energetica: row.get::<_, u8>(2)?,
                })
            })
            .expect("Errore nella lettura dei dati di tipo materiale")
            .collect();
        result.map_err(|e| e.to_string())
    }
}
