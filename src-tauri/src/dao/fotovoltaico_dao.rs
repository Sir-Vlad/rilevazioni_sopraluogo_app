use crate::app_traits::{CreateTable, DaoTrait, GetAll, Insert, Update};
use crate::entities::Fotovoltaico;
use crate::utils::AppError;

pub struct FotovoltaicoDAO;

impl DaoTrait for FotovoltaicoDAO {
    type Entity = Fotovoltaico;
    type Error = AppError;
}

impl CreateTable for FotovoltaicoDAO {}

impl GetAll for FotovoltaicoDAO {}

impl Insert for FotovoltaicoDAO {}

impl Update for FotovoltaicoDAO {}

#[cfg(test)]
mod test {
    use super::super::*;
    use crate::app_traits::{CreateTable, Insert, Update};
    use crate::entities::{Edificio, Fotovoltaico};
    use once_cell::sync::Lazy;
    use rusqlite::Connection;
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
    fn test_insert_data() {
        let conn = setup();

        let insert_data = Fotovoltaico::new("PR01-25", 55f32, "Ugo Ugolini");
        let result = FotovoltaicoDAO::insert(&conn, insert_data);
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
    fn test_update_data() {
        let conn = setup();

        let insert_data = Fotovoltaico::new("PR01-25", 55f32, "Ugo Ugolini");
        FotovoltaicoDAO::insert(&conn, insert_data).unwrap();

        let update_data = Fotovoltaico {
            id: 1,
            id_edificio: "PR01-25".to_string(),
            potenza: 85f32,
            proprietario: "Ugo Ugolini".to_string(),
        };
        pretty_sqlite::print_table(&conn, "fotovoltaico").expect("errore");
        let result = FotovoltaicoDAO::update(&conn, update_data.clone());
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
