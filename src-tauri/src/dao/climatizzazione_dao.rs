use crate::dao::crud_operations::GetAll;
use crate::dao::entity::Climatizzazione;
use crate::dao::schema_operations::CreateTable;
use crate::dao::utils::DAO;
use crate::database::{DatabaseConnection, QueryBuilder, SqlQueryBuilder};
use log::info;

pub struct ClimatizzazioneDAO;

impl DAO for ClimatizzazioneDAO {
    fn table_name() -> &'static str {
        "CLIMATIZZAZIONE"
    }
}

impl CreateTable for ClimatizzazioneDAO {
    fn create_table<C: DatabaseConnection>(conn: &C) -> Result<(), String> {
        conn.execute(
            format!(
                "CREATE TABLE IF NOT EXISTS {}
                (
                    ID                    INTEGER PRIMARY KEY AUTOINCREMENT,
                    CLIMATIZZAZIONE       TEXT    NOT NULL UNIQUE COLLATE NOCASE,
                    EFFICIENZA_ENERGETICA INTEGER NOT NULL
                ) STRICT;",
                Self::table_name()
            )
            .as_str(),
            (),
        )
        .map_err(|e| e.to_string())?;
        info!("Tabella CLIMATIZZAZIONE creata");
        Ok(())
    }
}

impl GetAll<Climatizzazione> for ClimatizzazioneDAO {
    fn get_all<C: DatabaseConnection>(conn: &C) -> Result<Vec<Climatizzazione>, String> {
        let (query, _) = QueryBuilder::select()
            .table(Self::table_name())
            .build()
            .map_err(|e| e.to_string())?;

        let mut stmt = conn
            .prepare(query.as_str())
            .map_err(|e| format!("Errore nella creazione della query: {}", e))?;

        let result: Result<Vec<Climatizzazione>, rusqlite::Error> = stmt
            .query_map([], |row| {
                Ok(Climatizzazione {
                    id: row.get::<_, u64>("ID")?,
                    climatizzazione: row.get::<_, String>("CLIMATIZZAZIONE")?,
                    efficienza_energetica: row.get::<_, u8>("EFFICIENZA_ENERGETICA")?,
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
    use rusqlite::Connection;

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
}
