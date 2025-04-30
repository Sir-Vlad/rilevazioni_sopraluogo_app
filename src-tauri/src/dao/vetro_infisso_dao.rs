use crate::dao::crud_operations::GetAll;
use crate::dao::entity::VetroInfisso;
use crate::dao::utils::schema_operations::CreateTable;
use crate::database::{DatabaseConnection, QueryBuilder, SqlQueryBuilder};
use log::info;

pub struct VetroInfissoDAO;

impl GetAll<VetroInfisso> for VetroInfissoDAO {
    fn get_all<C: DatabaseConnection>(conn: &C) -> Result<Vec<VetroInfisso>, String> {
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

impl CreateTable for VetroInfissoDAO {
    fn create_table<C: DatabaseConnection>(conn: &C) -> Result<(), String> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS VETRO_INFISSO
            (
                ID                    INTEGER PRIMARY KEY AUTOINCREMENT,
                VETRO                 TEXT    NOT NULL UNIQUE COLLATE NOCASE,
                EFFICIENZA_ENERGETICA INTEGER NOT NULL
            ) STRICT;",
            (),
        )
        .map_err(|e| e.to_string())?;
        info!("Tabella VETRO_INFISSO creata");
        Ok(())
    }
}
