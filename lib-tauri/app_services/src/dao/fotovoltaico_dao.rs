use crate::dao::utils::DAO;
use crate::database::WhereBuilder;
use crate::utils::AppError;
use crate::{
    dao::crud_operations::{GetAll, Insert, Update},
    dao::entity::Fotovoltaico,
    dao::schema_operations::CreateTable,
    database::{convert_param, DatabaseConnection, QueryBuilder, SqlQueryBuilder},
};
use log::{error, info};
use rusqlite::Error;

pub struct FotovoltaicoDAO;

impl DAO for FotovoltaicoDAO {
    fn table_name() -> &'static str {
        "FOTOVOLTAICO"
    }
}

impl CreateTable for FotovoltaicoDAO {
    fn create_table<C: DatabaseConnection>(conn: &C) -> Result<(), AppError> {
        conn.execute(
            format!(
                "CREATE TABLE IF NOT EXISTS {}
                (
                    ID           INTEGER PRIMARY KEY AUTOINCREMENT,
                    ID_EDIFICIO  TEXT REFERENCES EDIFICIO (CHIAVE),
                    POTENZA      REAL NOT NULL CHECK ( POTENZA >= 0 ),
                    PROPRIETARIO TEXT NOT NULL
                ) STRICT;",
                Self::table_name()
            )
            .as_str(),
            (),
        )?;
        info!("Tabella FOTOVOLTAICO creata");
        Ok(())
    }
}

impl GetAll<Fotovoltaico> for FotovoltaicoDAO {
    fn get_all<C: DatabaseConnection>(conn: &C) -> Result<Vec<Fotovoltaico>, AppError> {
        let (query, _) = QueryBuilder::select().table(Self::table_name()).build()?;
        let mut stmt = conn.prepare(query.as_str())?;
        let results: Result<Vec<Fotovoltaico>, Error> = stmt
            .query_map([], |row| {
                Ok(Fotovoltaico {
                    id: row.get("ID")?,
                    id_edificio: row.get("ID_EDIFICIO")?,
                    potenza: row.get("POTENZA")?,
                    proprietario: row.get("PROPRIETARIO")?,
                })
            })?
            .collect();
        results.map_err(AppError::from)
    }
}

impl Insert<Fotovoltaico> for FotovoltaicoDAO {
    fn insert<C: DatabaseConnection>(
        conn: &C,
        item: Fotovoltaico,
    ) -> Result<Fotovoltaico, AppError> {
        let builder = QueryBuilder::insert()
            .table(Self::table_name())
            .columns(vec!["ID_EDIFICIO", "POTENZA", "PROPRIETARIO"])
            .values(vec![
                item.id_edificio.clone().into(),
                item.potenza.into(),
                item.proprietario.clone().into(),
            ])
            .returning("ID");
        let (query, params) = builder.build()?;
        let mut stmt = conn.prepare(query.as_str())?;
        let mut results = stmt
            .query_map(rusqlite::params_from_iter(convert_param(params)), |row| {
                row.get::<_, u64>(0)
            })?;
        let id = results.next().unwrap()?;
        info!("Fotovoltaico {} inserito con successo", id);
        Ok(Fotovoltaico { id, ..item })
    }
}
impl Update<Fotovoltaico> for FotovoltaicoDAO {
    fn update<C: DatabaseConnection>(
        conn: &C,
        item: Fotovoltaico,
    ) -> Result<Fotovoltaico, AppError> {
        let builder = QueryBuilder::update()
            .table(Self::table_name())
            .set("POTENZA", item.potenza)
            .set("PROPRIETARIO", item.proprietario.clone())
            .where_eq("ID", item.id);
        let (query, params) = builder.build()?;
        let mut stmt = conn.prepare(query.as_str())?;
        let results = stmt.query_map(rusqlite::params_from_iter(convert_param(params)), |row| {
            row.get::<_, u64>(0)
        });
        match results {
            Ok(_) => {
                info!("Fotovoltaico {} aggiornato con successo", item.id);
                Ok(item)
            }
            Err(e) => {
                error!("Fotovoltaico {} non aggiornato: {e}", item.id);
                Err(AppError::from(e))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::*;
    use crate::dao::crud_operations::{Insert, Update};
    use crate::dao::entity::{Edificio, Fotovoltaico};
    use crate::dao::utils::schema_operations::CreateTable;
    use once_cell::sync::Lazy;
    use rusqlite::Connection;
    use serial_test::serial;
    use std::ops::Deref;
    use std::sync::Mutex;

    static DATABASE: Lazy<Mutex<Connection>> = Lazy::new(|| Mutex::new(setup()));

    fn setup() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        EdificioDAO::create_table(&conn).expect("Errore nella creazione della tabella Edificio");
        FotovoltaicoDAO::create_table(&conn)
            .expect("Errore nella creazione della tabella Fotovoltaico");

        let edificio = Edificio::new("PR01-25", "00008545", "Via Pallone");
        EdificioDAO::insert(&conn, edificio).expect("Errore nella creazione dell'edificio");
        conn
    }

    #[test]
    fn test_create_table() {
        let conn = Connection::open_in_memory().unwrap();

        let res = FotovoltaicoDAO::create_table(&conn);
        assert!(res.is_ok());
    }

    #[test]
    #[serial]
    fn test_insert_data() {
        let conn = DATABASE.lock().unwrap();

        let insert_data = Fotovoltaico::new("PR01-25", 55f32, "Ugo Ugolini");
        let result = FotovoltaicoDAO::insert(conn.deref(), insert_data);
        match result {
            Ok(res) => {
                pretty_sqlite::print_table(&conn, "fotovoltaico").expect("errore");
                assert_eq!(
                    res,
                    Fotovoltaico {
                        id: 1,
                        id_edificio: "PR01-25".to_string(),
                        potenza: 55f32,
                        proprietario: "Ugo Ugolini".to_string(),
                    }
                )
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    #[test]
    #[serial]
    fn test_update_data() {
        let conn = DATABASE.lock().unwrap();
        let update_data = Fotovoltaico {
            id: 1,
            id_edificio: "PR01-25".to_string(),
            potenza: 85f32,
            proprietario: "Ugo Ugolini".to_string(),
        };
        pretty_sqlite::print_table(&conn, "fotovoltaico").expect("errore");
        let result = FotovoltaicoDAO::update(conn.deref(), update_data.clone());
        match result {
            Ok(res) => {
                pretty_sqlite::print_table(&conn, "fotovoltaico").expect("errore");
                assert_eq!(res, update_data)
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
}
