use crate::dao::entity::MaterialeInfisso;
use rusqlite::Connection;

pub trait MaterialeInfissoDao {
    fn get_all(conn: &Connection) -> Result<Vec<MaterialeInfisso>, String>;
}

pub struct MaterialeInfissoDaoImpl;

impl MaterialeInfissoDao for MaterialeInfissoDaoImpl {
    fn get_all(conn: &Connection) -> Result<Vec<MaterialeInfisso>, String> {
        let mut stmt = conn
            .prepare("SELECT * FROM MATERIALE_INFISSO")
            .map_err(|e| format!("Errore nella creazione della query: {}", e))?;

        let result: Result<Vec<MaterialeInfisso>, rusqlite::Error> = stmt
            .query_map([], |row| {
                Ok(MaterialeInfisso {
                    id: row.get::<_, u64>(0)?,
                    materiale: row.get::<_, String>(1)?,
                    efficienza_energetica: row.get::<_, u8>(2)?,
                })
            })
            .expect("Errore nella lettura dei dati di tipo materiale")
            .collect();
        result.map_err(|e| e.to_string())
    }
}
