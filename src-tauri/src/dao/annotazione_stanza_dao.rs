use crate::dao::crud_operations::{GetAll, Insert};
use crate::dao::entity::AnnotazioneStanza;
use crate::dao::utils::schema_operations::CreateTable;
use crate::dao::utils::DAO;
use crate::database::{DatabaseConnection, QueryBuilder, SqlQueryBuilder};
use rusqlite::{params, Error};

pub struct AnnotazioneStanzaDAO;

impl DAO for AnnotazioneStanzaDAO {
    fn table_name() -> &'static str {
        "ANNOTAZIONE_STANZA"
    }
}

impl CreateTable for AnnotazioneStanzaDAO {
    fn create_table<C: DatabaseConnection>(conn: &C) -> Result<(), String> {
        conn.execute(
            format!(
                "CREATE TABLE IF NOT EXISTS {}
            (
                ID        INTEGER PRIMARY KEY AUTOINCREMENT,
                ID_STANZA INTEGER  NOT NULL REFERENCES STANZA (ID),
                CONTENT   TEXT     NOT NULL CHECK ( LENGTH(CONTENT > 0) ),
                DATA      DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            ) STRICT;",
                Self::table_name()
            )
            .as_str(),
            (),
        )?;
        Ok(())
    }
}

impl GetAll<AnnotazioneStanza> for AnnotazioneStanzaDAO {
    fn get_all<C: DatabaseConnection>(conn: &C) -> Result<Vec<AnnotazioneStanza>, String> {
        let (query, _) = QueryBuilder::select().table(Self::table_name()).build()?;
        let mut stmt = conn.prepare(query.as_str())?;
        let result: Result<Vec<AnnotazioneStanza>, Error> = stmt
            .query_map(params![], |row| {
                Ok(AnnotazioneStanza {
                    id: row.get("ID")?,
                    id_stanza: row.get("ID_STANZA")?,
                    content: row.get("CONTENT")?,
                    data: row.get("DATA")?,
                })
            })
            .map_err(|e| e.to_string())?
            .collect();
        result.map_err(|e| e.to_string())
    }
}

impl Insert<AnnotazioneStanza> for AnnotazioneStanzaDAO {
    fn insert<C: DatabaseConnection>(
        conn: &C,
        commento: AnnotazioneStanza,
    ) -> Result<AnnotazioneStanza, String> {
        todo!()
    }
}
