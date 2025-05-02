use crate::dao::crud_operations::{GetAll, Insert};
use crate::dao::entity::{Climatizzazione, MaterialeInfisso};
use crate::dao::utils::schema_operations::CreateTable;
use crate::dao::utils::DAO;
use crate::database::{convert_param, DatabaseConnection, QueryBuilder, SqlQueryBuilder};
use crate::utils::AppError;
use log::info;
use crate::dao::ClimatizzazioneDAO;

pub struct MaterialeInfissoDAO;

impl DAO for MaterialeInfissoDAO {
    fn table_name() -> &'static str {
        "MATERIALE_INFISSO"
    }
}

impl CreateTable for MaterialeInfissoDAO {
    fn create_table<C: DatabaseConnection>(conn: &C) -> Result<(), AppError> {
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
        )?;
        info!("Tabella MATERIALI_INFISSO creata");
        Ok(())
    }
}

impl GetAll<MaterialeInfisso> for MaterialeInfissoDAO {
    fn get_all<C: DatabaseConnection>(conn: &C) -> Result<Vec<MaterialeInfisso>, AppError> {
        let (query, _) = QueryBuilder::select().table("MATERIALE_INFISSO").build()?;

        let mut stmt = conn.prepare(query.as_str())?;

        let result: Result<Vec<MaterialeInfisso>, rusqlite::Error> = stmt
            .query_map([], |row| {
                Ok(MaterialeInfisso {
                    id: row.get::<_, u64>("ID")?,
                    materiale: row.get::<_, String>("MATERIALE")?,
                    efficienza_energetica: row.get::<_, u8>("EFFICIENZA_ENERGETICA")?,
                })
            })?
            .collect();
        result.map_err(|e| AppError::from(e))
    }
}

impl Insert<MaterialeInfisso> for MaterialeInfissoDAO {
    fn insert<C: DatabaseConnection>(
        conn: &C,
        item: MaterialeInfisso,
    ) -> Result<MaterialeInfisso, AppError> {
        let builder = QueryBuilder::insert()
            .table(Self::table_name())
            .columns(vec!["MATERIALE", "EFFICIENZA_ENERGETICA"])
            .values(vec![
                item.materiale.clone().into(),
                item.efficienza_energetica.into(),
            ])
            .returning("ID");
        let (query, param) = builder.build()?;
        let mut stmt = conn.prepare(query.as_str())?;
        let mut res = stmt.query_map(rusqlite::params_from_iter(convert_param(param)), |row| {
            row.get::<_, u64>(0)
        })?;
        let id = res.next().unwrap()?;
        Ok(MaterialeInfisso {
            id,
            materiale: item.materiale,
            efficienza_energetica: item.efficienza_energetica,
        })
    }
}