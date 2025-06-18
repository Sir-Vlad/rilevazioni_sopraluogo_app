use crate::dao::crud_operations::{GetAll, Insert};
use crate::dao::entity::VetroInfisso;
use crate::dao::utils::schema_operations::CreateTable;
use crate::dao::utils::DAO;
use crate::database::{convert_param, DatabaseConnection, QueryBuilder, SqlQueryBuilder};
use crate::utils::AppError;
use log::info;

pub struct VetroInfissoDAO;

impl DAO for VetroInfissoDAO {
    fn table_name() -> &'static str {
        "VETRO_INFISSO"
    }
}

impl CreateTable for VetroInfissoDAO {
    fn create_table<C: DatabaseConnection>(conn: &C) -> Result<(), AppError> {
        conn.execute(
            format!(
                "CREATE TABLE IF NOT EXISTS {}
                (
                    ID                    INTEGER PRIMARY KEY AUTOINCREMENT,
                    VETRO                 TEXT    NOT NULL UNIQUE COLLATE NOCASE,
                    EFFICIENZA_ENERGETICA INTEGER NOT NULL
                ) STRICT;",
                Self::table_name()
            )
            .as_str(),
            (),
        )?;
        info!("Tabella VETRO_INFISSO creata");
        Ok(())
    }
}

impl GetAll<VetroInfisso> for VetroInfissoDAO {
    fn get_all<C: DatabaseConnection>(conn: &C) -> Result<Vec<VetroInfisso>, AppError> {
        let (query, _) = QueryBuilder::select().table(Self::table_name()).build()?;

        let mut stmt = conn.prepare(query.as_str())?;

        let result: Result<Vec<VetroInfisso>, rusqlite::Error> = stmt
            .query_map([], |row| {
                Ok(VetroInfisso {
                    _id: Some(row.get::<_, u64>("ID")?),
                    vetro: row.get::<_, String>("VETRO")?,
                    efficienza_energetica: row.get::<_, u8>("EFFICIENZA_ENERGETICA")?,
                })
            })?
            .collect();
        result.map_err(AppError::from)
    }
}

impl Insert<VetroInfisso> for VetroInfissoDAO {
    fn insert<C: DatabaseConnection>(
        conn: &C,
        item: VetroInfisso,
    ) -> Result<VetroInfisso, AppError> {
        let builder = QueryBuilder::insert()
            .table(Self::table_name())
            .columns(vec!["VETRO", "EFFICIENZA_ENERGETICA"])
            .values(vec![
                item.vetro.clone().into(),
                item.efficienza_energetica.into(),
            ])
            .returning("ID");
        let (query, param) = builder.build()?;
        let mut stmt = conn.prepare(query.as_str())?;
        let mut res = stmt.query_map(rusqlite::params_from_iter(convert_param(param)), |row| {
            row.get::<_, u64>(0)
        })?;
        let id = res.next().unwrap()?;
        info!("VetroInfisso inserito con ID {}", item.vetro);
        Ok(VetroInfisso {
            _id: Some(id),
            ..item
        })
    }
}
