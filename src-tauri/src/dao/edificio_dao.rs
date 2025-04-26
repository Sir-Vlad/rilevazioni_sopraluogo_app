use crate::dao::Edificio;
use crate::database::{
    convert_param, DatabaseConnection, QueryBuilder, SqlQueryBuilder, WhereBuilder,
};
use log::{error, info};
use rusqlite::Connection;

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
        let (query, _) = QueryBuilder::select()
            .table("EDIFICIO")
            .build()
            .map_err(|e| e.to_string())?;

        let mut stmt = connection
            .prepare(query.as_str())
            .map_err(|e| format!("Errore nella creazione della query: {}", e))?;

        let result: Result<Vec<Edificio>, rusqlite::Error> = stmt
            .query_map([], |row| {
                Ok(Edificio {
                    chiave: row.get::<_, String>("CHIAVE")?,
                    fascicolo: row.get::<_, String>("FASCICOLO")?,
                    indirizzo: row.get::<_, String>("INDIRIZZO")?,
                    anno_costruzione: row.get::<_, Option<String>>("ANNO_COSTRUZIONE")?,
                    anno_riqualificazione: row.get::<_, Option<String>>("ANNO_RIQUALIFICAZIONE")?,
                    note_riqualificazione: row.get::<_, Option<String>>("NOTE_RIQUALIFICAZIONE")?,
                    isolamento_tetto: row.get::<_, Option<bool>>("ISOLAMENTO_TETTO")?,
                    cappotto: row.get::<_, Option<bool>>("CAPPOTTO")?,
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
        let builder = QueryBuilder::insert()
            .table("EDIFICIO")
            .columns(vec!["CHIAVE", "FASCICOLO", "INDIRIZZO"])
            .values(vec![
                edificio.chiave.clone().into(),
                edificio.fascicolo.clone().into(),
                edificio.indirizzo.clone().into(),
            ]);
        let (query, params) = builder.build().map_err(|e| e.to_string())?;

        match connection
            .execute(
                query.as_str(),
                rusqlite::params_from_iter(convert_param(params)),
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
        let builder = QueryBuilder::update()
            .table("EDIFICIO")
            .set("ANNO_COSTRUZIONE", edificio.anno_costruzione.clone())
            .set(
                "ANNO_RIQUALIFICAZIONE",
                edificio.anno_riqualificazione.clone(),
            )
            .set("ISOLAMENTO_TETTO", edificio.isolamento_tetto)
            .set("CAPPOTTO", edificio.cappotto)
            .where_eq("CHIAVE", edificio.chiave.clone());

        let (query, param) = builder.build().map_err(|e| e.to_string())?;

        match connection
            .execute(
                query.as_str(),
                rusqlite::params_from_iter(convert_param(param)),
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
