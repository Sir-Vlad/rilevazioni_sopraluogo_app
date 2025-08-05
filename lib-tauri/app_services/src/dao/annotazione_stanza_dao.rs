use crate::dao::crud_operations::{GetAll, Insert};
use crate::dao::entity::AnnotazioneStanza;
use crate::dao::utils::schema_operations::CreateTable;
use crate::dao::utils::{convert_timestamp_to_local, DAO};
use crate::database::{convert_param, DatabaseConnection, QueryBuilder, SqlQueryBuilder};
use crate::utils::AppError;
use rusqlite::{params, Error};

pub struct AnnotazioneStanzaDAO;

impl DAO for AnnotazioneStanzaDAO {
    fn table_name() -> &'static str {
        "ANNOTAZIONE_STANZA"
    }
}

impl CreateTable for AnnotazioneStanzaDAO {
    fn create_table<C: DatabaseConnection>(conn: &C) -> Result<(), AppError> {
        conn.execute(
            format!(
                "CREATE TABLE IF NOT EXISTS {}
                (
                    ID        INTEGER PRIMARY KEY AUTOINCREMENT,
                    ID_STANZA INTEGER  NOT NULL REFERENCES STANZA (ID),
                    CONTENT   TEXT     NOT NULL CHECK ( LENGTH(CONTENT) > 0 ),
                    DATA      TEXT     NOT NULL DEFAULT CURRENT_TIMESTAMP
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
    fn get_all<C: DatabaseConnection>(conn: &C) -> Result<Vec<AnnotazioneStanza>, AppError> {
        let (query, _) = QueryBuilder::select().table(Self::table_name()).build()?;
        let mut stmt = conn.prepare(query.as_str())?;
        let result: Result<Vec<AnnotazioneStanza>, Error> = stmt
            .query_map(params![], |row| {
                Ok(AnnotazioneStanza {
                    id: row.get("ID")?,
                    id_stanza: row.get("ID_STANZA")?,
                    content: row.get("CONTENT")?,
                    _data: row.get("DATA")?,
                })
            })?
            .collect();
        result.map_err(AppError::DatabaseError)
    }
}

impl Insert<AnnotazioneStanza> for AnnotazioneStanzaDAO {
    fn insert<C: DatabaseConnection>(
        conn: &C,
        item: AnnotazioneStanza,
    ) -> Result<AnnotazioneStanza, AppError> {
        let builder = QueryBuilder::insert()
            .table(Self::table_name())
            .columns(vec!["ID_STANZA", "CONTENT"])
            .values(vec![item.id_stanza.into(), item.content.clone().into()])
            .returning("ID, DATA");
        let (query, param) = builder.build()?;
        let mut stmt = conn.prepare(query.as_str())?;
        let (id, timestamp) = stmt
            .query_row(rusqlite::params_from_iter(convert_param(param)), |row| {
                Ok((row.get::<_, u64>(0)?, row.get::<_, String>(1)?))
            })?;

        Ok(AnnotazioneStanza {
            id,
            _data: Some(convert_timestamp_to_local(timestamp)?),
            ..item
        })
    }
}

#[cfg(test)]
mod test {
    use crate::dao::crud_operations::Insert;
    use crate::dao::entity::AnnotazioneStanza;
    use crate::dao::schema_operations::CreateTable;
    use crate::dao::utils::DAO;
    use crate::dao::AnnotazioneStanzaDAO;
    use crate::utils::AppError;
    use rusqlite::{ffi, Connection, Error, ErrorCode};

    fn setup() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute("PRAGMA foreign_keys = OFF;", []).unwrap();
        AnnotazioneStanzaDAO::create_table(&conn).unwrap();
        conn
    }

    #[test]
    fn test_insert() {
        let conn = setup();
        let item = AnnotazioneStanza {
            id: 0,
            id_stanza: 1,
            content: "test".to_string(),
            _data: Option::from("".to_string()),
        };
        let res = AnnotazioneStanzaDAO::insert(&conn, item.clone());

        match res {
            Ok(i) => {
                pretty_sqlite::print_table(&conn, AnnotazioneStanzaDAO::table_name()).unwrap();
                assert_eq!(i.id_stanza, item.id_stanza);
                assert_eq!(i.id_stanza, item.id_stanza);
            }
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }

    #[test]
    fn test_insert_content_empty() {
        let conn = setup();
        let item = AnnotazioneStanza {
            id: 0,
            id_stanza: 1,
            content: "".to_string(),
            _data: Option::from("".to_string()),
        };
        let res = AnnotazioneStanzaDAO::insert(&conn, item.clone());

        match res.err().unwrap() {
            AppError::DatabaseError(Error::SqliteFailure(
                ffi::Error {
                    code: ErrorCode::ConstraintViolation,
                    ..
                },
                res,
            )) => {
                assert!(res.unwrap().contains("LENGTH(CONTENT) > 0"));
            }
            e => panic!("{:?}", e),
        }
    }
}
