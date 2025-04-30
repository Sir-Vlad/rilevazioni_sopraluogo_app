use crate::dao::crud_operations::{GetAll, Insert, Update};
use crate::dao::entity::Utenza;
use crate::dao::utils::schema_operations::CreateTable;
use crate::database::{convert_param, DatabaseConnection, QueryBuilder, SqlQueryBuilder, WhereBuilder};
use log::info;

pub struct UtenzeDAO;

impl GetAll<Utenza> for UtenzeDAO {
    fn get_all<C: DatabaseConnection>(conn: &C) -> Result<Vec<Utenza>, String> {
        let (query, _) = QueryBuilder::select().table("UTENZE").build()?;
        let mut stmt = conn.prepare(query.as_str()).map_err(|e| e.to_string())?;
        let result: Result<Vec<Utenza>, rusqlite::Error> = stmt
            .query_map([], |row| {
                Ok(Utenza {
                    id: row.get("ID")?,
                    id_edificio: row.get("ID_EDIFICIO")?,
                    tipo: row.get::<_, String>("TIPO")?.into(),
                    cod_contatore: row.get("COD_CONTATORE")?,
                    indirizzo_contatore: row.get("INDIRIZZO_CONTATORE")?,
                })
            })
            .map_err(|e| e.to_string())?
            .collect();
        result.map_err(|e| e.to_string())
    }
}

impl Insert<Utenza> for UtenzeDAO {
    fn insert<C: DatabaseConnection>(conn: &C, item: Utenza) -> Result<Utenza, String> {
        let builder = QueryBuilder::insert()
            .table("UTENZE")
            .columns(vec![
                "ID_EDIFICIO",
                "TIPO",
                "COD_CONTATORE",
                "INDIRIZZO_CONTATORE",
            ])
            .values(vec![
                item.id_edificio.clone().into(),
                item.tipo.to_string().into(),
                item.cod_contatore.clone().into(),
                item.indirizzo_contatore.clone().into(),
            ])
            .returning("ID");
        let (query, param) = builder.build()?;
        let mut stmt = conn.prepare(query.as_str()).map_err(|e| e.to_string())?;
        let mut result = stmt
            .query_map(rusqlite::params_from_iter(convert_param(param)), |row| {
                row.get::<_, u64>(0)
            })
            .map_err(|e| e.to_string())?;
        let id = result.next().unwrap().unwrap();
        Ok(Utenza { id, ..item })
    }
}

impl Update<Utenza> for UtenzeDAO {
    fn update<C: DatabaseConnection>(conn: &C, item: Utenza) -> Result<Utenza, String> {
        let builder = QueryBuilder::update()
            .table("UTENZE")
            .set("COD_CONTATORE", item.cod_contatore.clone())
            .set_if("INDIRIZZO_CONTATORE", item.indirizzo_contatore.clone())
            .where_eq("ID", item.id);
        let (query, params) = builder.build()?;
        let res = conn
            .execute(
                query.as_str(),
                rusqlite::params_from_iter(convert_param(params)),
            )
            .map_err(|e| e.to_string());
        match res {
            Ok(_) => Ok(item),
            Err(e) => Err(e),
        }
    }
}

impl CreateTable for UtenzeDAO {
    fn create_table<C: DatabaseConnection>(conn: &C) -> Result<(), String> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS UTENZE
            (
                ID                  INTEGER PRIMARY KEY AUTOINCREMENT,
                ID_EDIFICIO         TEXT NOT NULL REFERENCES EDIFICIO (CHIAVE),
                TIPO                TEXT NOT NULL CHECK ( TIPO IN ('acqua', 'calore', 'elettricità') ),
                COD_CONTATORE       TEXT NOT NULL,
                INDIRIZZO_CONTATORE TEXT
            ) STRICT;",
            (),
        )
        .map_err(|e| e.to_string())?;
        info!("Table utenze creata");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::dao::crud_operations::{Insert, Update};
    use crate::dao::entity::{Edificio, TipoUtenza, Utenza};
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
        UtenzeDAO::create_table(&conn).expect("Errore nella creazione della tabella Utenze");

        let edificio = Edificio::new("PR01-25", "00008545", "Via Pallone");
        EdificioDAO::insert(&conn, edificio).expect("Errore nella creazione dell'edificio");
        conn
    }

    #[test]
    fn test_create_table() {
        let conn = Connection::open_in_memory().unwrap();

        let res = UtenzeDAO::create_table(&conn);
        assert!(res.is_ok());
    }

    #[test]
    #[serial]
    fn test_insert() {
        let conn = DATABASE.lock().unwrap();
        let utenza = Utenza::new("PR01-25", "acqua", "00008545", "Via Pallone");
        let result = UtenzeDAO::insert(conn.deref(), utenza);
        match result {
            Ok(res) => {
                pretty_sqlite::print_table(&conn, "UTENZE").unwrap();
                assert_eq!(
                    res,
                    Utenza {
                        id: 1,
                        id_edificio: "PR01-25".to_string(),
                        tipo: TipoUtenza::Idrica,
                        cod_contatore: "00008545".to_string(),
                        indirizzo_contatore: Option::from("Via Pallone".to_string()),
                    }
                )
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    #[serial]
    fn test_update() {
        let conn = DATABASE.lock().unwrap();
        let update_utenza = Utenza {
            id: 1,
            id_edificio: "PR01-25".to_string(),
            tipo: TipoUtenza::Idrica,
            cod_contatore: "00008545".to_string(),
            indirizzo_contatore: Option::from("Via Roma".to_string()),
        };
        pretty_sqlite::print_table(&conn, "UTENZE").unwrap();
        let res = UtenzeDAO::update(conn.deref(), update_utenza.clone());
        match res {
            Ok(res) => {
                pretty_sqlite::print_table(&conn, "UTENZE").unwrap();
                assert_eq!(res, update_utenza);
            }
            Err(e) => panic!("{}", e),
        }
    }
}
