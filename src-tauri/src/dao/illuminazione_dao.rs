use crate::dao::crud_operations::GetAll;
use crate::dao::entity::Illuminazione;
use crate::dao::utils::schema_operations::CreateTable;
use crate::dao::utils::DAO;
use crate::database::{DatabaseConnection, QueryBuilder, SqlQueryBuilder};
use log::info;

pub struct IlluminazioneDAO;

impl DAO for IlluminazioneDAO {
    fn table_name() -> &'static str {
        "ILLUMINAZIONE"
    }
}

impl CreateTable for IlluminazioneDAO {
    fn create_table<C: DatabaseConnection>(conn: &C) -> Result<(), String> {
        conn.execute(
            format!(
                "CREATE TABLE IF NOT EXISTS {}
                    (
                        ID                    INTEGER PRIMARY KEY AUTOINCREMENT,
                        LAMPADINA             TEXT    NOT NULL UNIQUE COLLATE NOCASE,
                        EFFICIENZA_ENERGETICA INTEGER NOT NULL
                    ) STRICT;",
                Self::table_name()
            )
            .as_str(),
            (),
        )
        .map_err(|e| e.to_string())?;
        info!("Tabella ILLUMINAZIONE creata");
        Ok(())
    }
}

impl GetAll<Illuminazione> for IlluminazioneDAO {
    fn get_all<C: DatabaseConnection>(conn: &C) -> Result<Vec<Illuminazione>, String> {
        let (query, _) = QueryBuilder::select()
            .table(Self::table_name())
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
