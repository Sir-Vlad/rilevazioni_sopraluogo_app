use crate::app_traits::{CreateTable, DaoTrait, GetAll, Insert, Update};
use crate::entities::Infisso;
use crate::utils::AppError;

pub struct InfissoDAO;

impl DaoTrait for InfissoDAO {
    type Entity = Infisso;
    type Error = AppError;
}
impl CreateTable for InfissoDAO {}
impl GetAll for InfissoDAO {}
impl Insert for InfissoDAO {}
impl Update for InfissoDAO {}

#[cfg(test)]
mod test {
    use super::*;
    use crate::app_traits::EntityTrait;
    use rusqlite::Connection;

    fn setup() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.pragma_update(None, "foreign_keys", "OFF").unwrap();

        InfissoDAO::create_table(&conn).unwrap();
        conn
    }

    #[test]
    fn test_insert() {
        let conn = setup();

        let entity = Infisso {
            id: "A".to_string(),
            edificio_id: "1587".to_string(),
            tipo: "Porta".to_string(),
            altezza: 150,
            larghezza: 425,
            materiale: "Legno".to_string(),
            vetro: "Singolo".to_string(),
        };

        match InfissoDAO::insert(&conn, entity.clone()) {
            Ok(res) => {
                assert_eq!(res, entity);
            }
            Err(err) => panic!("Errore durante l'inserimento: {err}"),
        }

        pretty_sqlite::print_table(&conn, &Infisso::table_name()).unwrap()
    }

    #[test]
    fn test_update() {
        let conn = setup();

        let mut entity = Infisso {
            id: "A".to_string(),
            edificio_id: "1587".to_string(),
            tipo: "Porta".to_string(),
            altezza: 150,
            larghezza: 425,
            materiale: "Legno".to_string(),
            vetro: "Singolo".to_string(),
        };

        InfissoDAO::insert(&conn, entity.clone()).unwrap();
        pretty_sqlite::print_table(&conn, &Infisso::table_name()).unwrap();

        entity.altezza = 171;
        entity.materiale = "Muro".to_string();

        match InfissoDAO::update(&conn, entity.clone()) {
            Ok(res) => {
                assert_eq!(res, entity);
            }
            Err(err) => panic!("Errore durante l'aggiornamento: {err}"),
        }

        pretty_sqlite::print_table(&conn, &Infisso::table_name()).unwrap()
    }
}
