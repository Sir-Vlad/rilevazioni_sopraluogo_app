use crate::dao::entities::entity::Illuminazione;
use rusqlite::Connection;

pub trait IlluminazioneDAO {
    fn get_all(conn: &Connection) -> Result<Vec<Illuminazione>, String>;
}

pub struct IlluminazioneDAOImpl;

impl IlluminazioneDAO for IlluminazioneDAOImpl {
    fn get_all(conn: &Connection) -> Result<Vec<Illuminazione>, String> {
        let mut stmt = conn
            .prepare("SELECT * FROM ILLUMINAZIONE")
            .map_err(|e| format!("Errore nella creazione della query: {}", e))?;

        let result: Result<Vec<Illuminazione>, rusqlite::Error> = stmt
            .query_map([], |row| {
                Ok(Illuminazione {
                    id: row.get::<_, u64>(0)?,
                    lampadina: row.get::<_, String>(1)?,
                    efficienza_energetica: row.get::<_, u8>(2)?,
                })
            })
            .expect("Errore nella lettura dei dati di tipo materiale")
            .collect();
        result.map_err(|e| e.to_string())
    }
}
