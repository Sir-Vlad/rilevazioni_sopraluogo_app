use crate::app_traits::{CreateTable, DaoTrait, GetAll, Insert};
use crate::dao::entity::Climatizzazione;
use crate::database::{DatabaseConnection, SqlQueryBuilder};
use crate::utils::AppError;

pub struct ClimatizzazioneDAO;

impl DaoTrait for ClimatizzazioneDAO {
    type Entity = Climatizzazione;
    type Error = AppError;
}

impl CreateTable for ClimatizzazioneDAO {}
impl GetAll for ClimatizzazioneDAO {}

impl Insert for ClimatizzazioneDAO {}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        ClimatizzazioneDAO::create_table(&conn).unwrap();
        conn
    }

    fn insert_test_data(conn: &Connection) -> Vec<Climatizzazione> {
        let test_data = vec![(1, "Neon", 4), (2, "Led", 3), (3, "Fluorescenza", 2)];

        let mut expected_results = Vec::new();

        for (id, climatizzazione, efficienza) in test_data {
            conn.execute(
                "INSERT INTO CLIMATIZZAZIONE (id, climatizzazione, efficienza_energetica) 
                 VALUES (?1, ?2, ?3)",
                [&id.to_string(), climatizzazione, &efficienza.to_string()],
            )
            .unwrap();

            expected_results.push(Climatizzazione {
                _id: Some(id),
                climatizzazione: climatizzazione.to_string(),
                efficienza_energetica: efficienza,
            });
        }

        expected_results
    }

    #[test]
    fn get_all() {
        let conn = setup_db();
        let excepted_data = insert_test_data(&conn);
        let actual_data = ClimatizzazioneDAO::get_all(&conn).unwrap();

        assert_eq!(actual_data.len(), excepted_data.len());

        for (actual, expected) in actual_data.iter().zip(excepted_data.iter()) {
            assert_eq!(actual, expected);
        }
    }
    #[test]
    fn get_all_empty_table() {
        // Setup - solo creazione del database senza inserimento dati
        let conn = setup_db();

        // Test
        let risultati = ClimatizzazioneDAO::get_all(&conn).unwrap();

        // Verifica
        assert!(risultati.is_empty());
    }

    #[test]
    fn test_errore_tabella_non_esistente() {
        // Creare un database in memoria senza creare la tabella CLIMATIZZAZIONE
        let conn = Connection::open_in_memory().unwrap();

        // Il metodo dovrebbe restituire un errore
        let risultato = ClimatizzazioneDAO::get_all(&conn);
        assert!(risultato.is_err());
    }

    #[test]
    fn test_insert() {
        let conn = setup_db();

        let entity = Climatizzazione {
            _id: None,
            climatizzazione: "Test".to_string(),
            efficienza_energetica: 2,
        };
        let result = ClimatizzazioneDAO::insert(&conn, entity.clone());
        match result {
            Ok(res) => {
                assert_eq!(1, res._id.unwrap());
            }
            Err(err) => panic!("Insert failed: {}", err),
        }
    }
}
