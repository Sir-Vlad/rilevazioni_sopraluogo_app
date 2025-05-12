use crate::dao::crud_operations::{GetAll, Insert};
use crate::dao::entity::TipoInfisso;
use crate::dao::schema_operations::CreateTable;
use crate::dao::utils::DAO;
use crate::database::{DatabaseConnection, QueryBuilder, SqlQueryBuilder};
use crate::utils::AppError;
use log::info;
use rusqlite::params;

pub struct TipoInfissoDAO;

impl DAO for TipoInfissoDAO {
    fn table_name() -> &'static str {
        "TIPO_INFISSO"
    }
}

impl CreateTable for TipoInfissoDAO {
    fn create_table<C: DatabaseConnection>(conn: &C) -> Result<(), AppError> {
        conn.execute(
            format!(
                "CREATE TABLE IF NOT EXISTS {}
                (
                    ID   INTEGER PRIMARY KEY,
                    NOME TEXT NOT NULL UNIQUE
                )
                ",
                Self::table_name()
            )
            .as_str(),
            (),
        )?;
        info!("Tabella TIPO_INFISSO creata");
        Ok(())
    }
}

impl GetAll<TipoInfisso> for TipoInfissoDAO {
    fn get_all<C: DatabaseConnection>(conn: &C) -> Result<Vec<TipoInfisso>, AppError> {
        let (query, _) = QueryBuilder::select().table(Self::table_name()).build()?;
        let mut stmt = conn.prepare(query.as_str())?;
        let results = stmt
            .query_map([], |row| {
                Ok(TipoInfisso {
                    id: row.get("ID")?,
                    nome: row.get("NOME")?,
                })
            })?
            .collect::<Result<Vec<_>, _>>();
        results.map_err(AppError::from)
    }
}

impl Insert<TipoInfisso> for TipoInfissoDAO {
    fn insert<C: DatabaseConnection>(conn: &C, item: TipoInfisso) -> Result<TipoInfisso, AppError> {
        let query = format!(
            "INSERT OR IGNORE INTO {}(NOME) VALUES (?1) RETURNING ID",
            Self::table_name()
        );
        let mut stmt = conn.prepare(query.as_str())?;
        let mut res = stmt.query_map(params![item.nome], |row| row.get::<_, u64>(0))?;
        let id = res.next().unwrap()?;
        Ok(TipoInfisso { id, ..item })
    }
}
