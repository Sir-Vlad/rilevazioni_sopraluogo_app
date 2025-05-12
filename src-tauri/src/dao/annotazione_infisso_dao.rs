use crate::dao::crud_operations::{GetAll, Insert};
use crate::dao::entity::AnnotazioneInfisso;
use crate::dao::utils::schema_operations::CreateTable;
use crate::dao::utils::{convert_timestamp_to_local, DAO};
use crate::database::{convert_param, DatabaseConnection, QueryBuilder, SqlQueryBuilder};
use crate::utils::AppError;
use rusqlite::{params, Error};

pub struct AnnotazioneInfissoDAO;

impl DAO for AnnotazioneInfissoDAO {
    fn table_name() -> &'static str {
        "ANNOTAZIONE_INFISSO"
    }
}

impl CreateTable for AnnotazioneInfissoDAO {
    fn create_table<C: DatabaseConnection>(conn: &C) -> Result<(), AppError> {
        conn.execute(
            format!(
                "CREATE TABLE IF NOT EXISTS {}
                (
                    ID         INTEGER PRIMARY KEY AUTOINCREMENT,
                    ID_INFISSO TEXT NOT NULL REFERENCES INFISSO (ID),
                    CONTENT    TEXT NOT NULL CHECK ( length(CONTENT) > 0 ),
                    DATA       TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
                ) STRICT;",
                Self::table_name()
            )
            .as_str(),
            (),
        )?;
        Ok(())
    }
}

impl GetAll<AnnotazioneInfisso> for AnnotazioneInfissoDAO {
    fn get_all<C: DatabaseConnection>(conn: &C) -> Result<Vec<AnnotazioneInfisso>, AppError> {
        let (query, _) = QueryBuilder::select().table(Self::table_name()).build()?;
        let mut stmt = conn.prepare(query.as_str())?;
        let result: Result<Vec<AnnotazioneInfisso>, Error> = stmt
            .query_map(params![], |row| {
                Ok(AnnotazioneInfisso {
                    id: row.get("ID")?,
                    id_infisso: row.get("ID_STANZA")?,
                    content: row.get("CONTENT")?,
                    data: row.get("DATA")?,
                })
            })?
            .collect();
        result.map_err(AppError::DatabaseError)
    }
}

impl Insert<AnnotazioneInfisso> for AnnotazioneInfissoDAO {
    fn insert<C: DatabaseConnection>(
        conn: &C,
        item: AnnotazioneInfisso,
    ) -> Result<AnnotazioneInfisso, AppError> {
        let builder = QueryBuilder::insert()
            .table(Self::table_name())
            .columns(vec!["ID_INFISSO", "CONTENT"])
            .values(vec![
                item.id_infisso.clone().into(),
                item.content.clone().into(),
            ])
            .returning("ID, DATA");
        let (query, param) = builder.build()?;
        let mut stmt = conn.prepare(query.as_str())?;
        let (id, timestamp) = stmt
            .query_row(rusqlite::params_from_iter(convert_param(param)), |row| {
                Ok((row.get::<_, u64>(0)?, row.get::<_, String>(1)?))
            })?;

        Ok(AnnotazioneInfisso {
            id,
            data: Some(convert_timestamp_to_local(timestamp)?),
            ..item
        })
    }
}
