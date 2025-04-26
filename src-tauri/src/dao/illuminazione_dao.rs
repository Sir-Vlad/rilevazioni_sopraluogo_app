use crate::dao::entity::Illuminazione;
use crate::database::{QueryBuilder, SqlQueryBuilder};
use rusqlite::Connection;

pub trait IlluminazioneDAO {
    fn get_all(conn: &Connection) -> Result<Vec<Illuminazione>, String>;
}

pub struct IlluminazioneDAOImpl;

impl IlluminazioneDAO for IlluminazioneDAOImpl {
    fn get_all(conn: &Connection) -> Result<Vec<Illuminazione>, String> {
        let (query, _) = QueryBuilder::select()
            .table("ILLUMINAZIONE")
            .build()
            .map_err(|e| e.to_string())?;

        let mut stmt = conn
            .prepare(query.as_str())
            .map_err(|e| format!("Errore nella creazione della query: {}", e))?;

        let result: Result<Vec<Illuminazione>, rusqlite::Error> = stmt
            .query_map([], |row| {
                Ok(Illuminazione {
                    id: row.get::<_, u64>("ID")?,
                    lampadina: row.get::<_, String>("LAMPADINA")?,
                    efficienza_energetica: row.get::<_, u8>("EFFICIENZA_ENERGETICA")?,
                })
            })
            .expect("Errore nella lettura dei dati di tipo materiale")
            .collect();
        result.map_err(|e| e.to_string())
    }
}
