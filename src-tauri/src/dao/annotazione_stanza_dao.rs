use crate::app_traits::{CreateTable, DaoTrait, GetAll, Insert};
use crate::entities::AnnotazioneStanza;
use crate::utils::AppError;

pub struct AnnotazioneStanzaDAO;

impl DaoTrait for AnnotazioneStanzaDAO {
    type Entity = AnnotazioneStanza;
    type Error = AppError;
}

impl CreateTable for AnnotazioneStanzaDAO {}

impl GetAll for AnnotazioneStanzaDAO {}
impl Insert for AnnotazioneStanzaDAO {}

#[cfg(test)]
mod test {
    use crate::app_traits::{CreateTable, DaoTrait, Insert};
    use crate::entities::AnnotazioneStanza;
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
                pretty_sqlite::print_table(&conn, &AnnotazioneStanzaDAO::table_name()).unwrap();
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
