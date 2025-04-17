use crate::dao::Edificio;
use crate::database::DatabaseConnection;
use log::{error, info};
use rusqlite::{params, Connection};

pub trait EdificioDAO {
    fn get_all(connection: &Connection) -> Result<Vec<Edificio>, String>;
    fn insert<C: DatabaseConnection>(
        connection: &C,
        edificio: Edificio,
    ) -> Result<Edificio, String>;
    fn update<C: DatabaseConnection>(
        connection: &C,
        edificio: Edificio,
    ) -> Result<Edificio, String>;
}

pub struct EdificioDAOImpl;

impl EdificioDAO for EdificioDAOImpl {
    fn get_all(connection: &Connection) -> Result<Vec<Edificio>, String> {
        let mut stmt = connection
            .prepare("SELECT * FROM Edificio")
            .map_err(|e| format!("Errore nella creazione della query: {}", e))?;

        let result: Result<Vec<Edificio>, rusqlite::Error> = stmt
            .query_map([], |row| {
                Ok(Edificio {
                    chiave: row.get::<_, String>(0)?,
                    fascicolo: row.get::<_, String>(1)?,
                    indirizzo: row.get::<_, String>(2)?,
                    anno_costruzione: row.get::<_, Option<String>>(3)?,
                    anno_riqualificazione: row.get::<_, Option<String>>(4)?,
                    note_riqualificazione: row.get::<_, Option<String>>(5)?,
                    isolamento_tetto: row.get::<_, Option<bool>>(6)?,
                    cappotto: row.get::<_, Option<bool>>(7)?,
                })
            })
            .expect("Errore nella lettura dei dati di tipo materiale")
            .collect();
        result.map_err(|e| e.to_string())
    }

    fn insert<C: DatabaseConnection>(
        connection: &C,
        edificio: Edificio,
    ) -> Result<Edificio, String> {
        match connection
            .execute(
                "INSERT INTO EDIFICIO(CHIAVE, FASCICOLO, INDIRIZZO)
                    VALUES (?1, ?2, ?3)",
                params![edificio.chiave, edificio.fascicolo, edificio.indirizzo],
            )
            .map_err(|e| e.to_string())
        {
            Ok(_) => {
                info!("Edificio inserito con successo");
                Ok(edificio)
            }
            Err(e) => {
                error!("Errore durante l'inserimento {{ edificio }}: {}", e);
                Err(e)
            }
        }
    }

    fn update<C: DatabaseConnection>(
        connection: &C,
        edificio: Edificio,
    ) -> Result<Edificio, String> {
        match connection
            .execute(
                "UPDATE EDIFICIO
                    SET anno_costruzione      = ?1,
                        anno_riqualificazione = ?2,
                        isolamento_tetto      = ?3,
                        cappotto              = ?4
                    WHERE chiave = ?5
            ",
                params![
                    edificio.anno_costruzione,
                    edificio.anno_riqualificazione,
                    edificio.isolamento_tetto,
                    edificio.cappotto,
                    edificio.chiave,
                ],
            )
            .map_err(|e| e.to_string())
        {
            Ok(_) => {
                info!("Edificio aggiornato con successo");
                Ok(edificio)
            }
            Err(e) => {
                error!("Errore durante l'aggiornamento {{ edificio }}: {}", e);
                Err(e)
            }
        }
    }
}
