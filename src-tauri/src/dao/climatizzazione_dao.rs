use crate::dao::entities::entity::Climatizzazione;
use rusqlite::Connection;

pub trait ClimatizzazioneDAO {
    fn get_all(conn: &Connection) -> Result<Vec<Climatizzazione>, String>;
}

pub struct ClimatizzazioneDAOImpl;

impl ClimatizzazioneDAO for ClimatizzazioneDAOImpl {
    fn get_all(conn: &Connection) -> Result<Vec<Climatizzazione>, String> {
        let mut stmt = conn
            .prepare("SELECT * FROM CLIMATIZZAZIONE")
            .map_err(|e| format!("Errore nella creazione della query: {}", e))?;

        let result: Result<Vec<Climatizzazione>, rusqlite::Error> = stmt
            .query_map([], |row| {
                Ok(Climatizzazione {
                    id: row.get::<_, u64>(0)?,
                    climatizzazione: row.get::<_, String>(1)?,
                    efficienza_energetica: row.get::<_, u8>(2)?,
                })
            })
            .expect("Errore nella lettura dei dati di tipo materiale")
            .collect();
        result.map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE CLIMATIZZAZIONE (
                ID INTEGER PRIMARY KEY,
                CLIMATIZZAZIONE TEXT NOT NULL,
                EFFICIENZA_ENERGETICA INTEGER NOT NULL
            )",
            [],
        )
        .unwrap();

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
                id,
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
        let actual_data = ClimatizzazioneDAOImpl::get_all(&conn).unwrap();

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
        let risultati = ClimatizzazioneDAOImpl::get_all(&conn).unwrap();

        // Verifica
        assert!(risultati.is_empty());
    }

    #[test]
    fn test_errore_tabella_non_esistente() {
        // Creare un database in memoria senza creare la tabella CLIMATIZZAZIONE
        let conn = Connection::open_in_memory().unwrap();

        // Il metodo dovrebbe restituire un errore
        let risultato = ClimatizzazioneDAOImpl::get_all(&conn);
        assert!(risultato.is_err());
    }
}
