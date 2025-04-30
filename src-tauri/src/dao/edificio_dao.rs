use crate::dao::crud_operations::{GetAll, Insert, Update};
use crate::dao::entity::Edificio;
use crate::dao::utils::schema_operations::CreateTable;
use crate::database::{
    convert_param, DatabaseConnection, QueryBuilder, SqlQueryBuilder, WhereBuilder,
};
use log::{error, info};

pub struct EdificioDAO;

impl CreateTable for EdificioDAO {
    fn create_table<C: DatabaseConnection>(conn: &C) -> Result<(), String> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS EDIFICIO
        (
            CHIAVE                TEXT PRIMARY KEY,
            FASCICOLO             TEXT NOT NULL,
            INDIRIZZO             TEXT NOT NULL,
            ANNO_COSTRUZIONE      TEXT    DEFAULT NULL,
            ANNO_RIQUALIFICAZIONE TEXT    DEFAULT NULL,
            NOTE_RIQUALIFICAZIONE TEXT    DEFAULT NULL,
            ISOLAMENTO_TETTO      INTEGER DEFAULT FALSE,
            CAPPOTTO              INTEGER DEFAULT FALSE
        ) STRICT;",
            (),
        )
        .map_err(|e| e.to_string())?;
        info!("Tabella EDIFICIO creata");
        Ok(())
    }
}

impl GetAll<Edificio> for EdificioDAO {
    fn get_all<C: DatabaseConnection>(connection: &C) -> Result<Vec<Edificio>, String> {
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
}

impl Insert<Edificio> for EdificioDAO {
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
}

impl Update<Edificio> for EdificioDAO {
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
