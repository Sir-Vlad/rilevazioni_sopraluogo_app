use crate::dao::crud_operations::GetAll;
use crate::dao::entity::MaterialeInfisso;
use crate::dao::utils::schema_operations::CreateTable;
use crate::dao::utils::DAO;
use crate::database::{DatabaseConnection, QueryBuilder, SqlQueryBuilder};
use log::info;

pub struct MaterialeInfissoDAO;

impl DAO for MaterialeInfissoDAO {
    fn table_name() -> &'static str {
        "MATERIALE_INFISSO"
    }
}

impl CreateTable for MaterialeInfissoDAO {
    fn create_table<C: DatabaseConnection>(conn: &C) -> Result<(), String> {
        conn.execute(
            format!(
                "CREATE TABLE IF NOT EXISTS {}
                (
                    ID                    INTEGER PRIMARY KEY AUTOINCREMENT,
                    MATERIALE             TEXT    NOT NULL UNIQUE COLLATE NOCASE,
                    EFFICIENZA_ENERGETICA INTEGER NOT NULL
                ) STRICT;",
                Self::table_name()
            )
            .as_str(),
            (),
        )
        .map_err(|e| e.to_string())?;
        info!("Tabella MATERIALI_INFISSO creata");
        Ok(())
    }
}

impl GetAll<MaterialeInfisso> for MaterialeInfissoDAO {
    fn get_all<C: DatabaseConnection>(conn: &C) -> Result<Vec<MaterialeInfisso>, String> {
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
