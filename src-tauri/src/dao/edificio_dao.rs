use crate::dao::crud_operations::{GetAll, Insert, Update};
use crate::dao::entity::Edificio;
use crate::dao::utils::schema_operations::CreateTable;
use crate::dao::utils::DAO;
use crate::database::{
    convert_param, DatabaseConnection, QueryBuilder, SqlQueryBuilder, WhereBuilder,
};
use crate::utils::AppError;
use log::{error, info};

pub struct EdificioDAO;

impl DAO for EdificioDAO {
    fn table_name() -> &'static str {
        "EDIFICIO"
    }
}

impl CreateTable for EdificioDAO {
    fn create_table<C: DatabaseConnection>(conn: &C) -> Result<(), AppError> {
        conn.execute(
            format!(
                "CREATE TABLE IF NOT EXISTS {}
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
                Self::table_name()
            )
            .as_str(),
            (),
        )?;
        info!("Tabella EDIFICIO creata");
        Ok(())
    }
}

impl GetAll<Edificio> for EdificioDAO {
    fn get_all<C: DatabaseConnection>(conn: &C) -> Result<Vec<Edificio>, AppError> {
        let (query, _) = QueryBuilder::select().table(Self::table_name()).build()?;

        let mut stmt = conn.prepare(query.as_str())?;

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
            })?
            .collect();
        result.map_err(AppError::from)
    }
}

impl Insert<Edificio> for EdificioDAO {
    fn insert<C: DatabaseConnection>(conn: &C, item: Edificio) -> Result<Edificio, AppError> {
        let builder = QueryBuilder::insert()
            .table(Self::table_name())
            .columns(vec!["CHIAVE", "FASCICOLO", "INDIRIZZO"])
            .values(vec![
                item.chiave.clone().into(),
                item.fascicolo.clone().into(),
                item.indirizzo.clone().into(),
            ]);
        let (query, params) = builder.build()?;

        match conn.execute(
            query.as_str(),
            rusqlite::params_from_iter(convert_param(params)),
        ) {
            Ok(_) => {
                info!("Edificio inserito con successo");
                Ok(item)
            }
            Err(e) => {
                error!("Errore durante l'inserimento {{ edificio }}: {}", e);
                Err(e)
            }
        }
    }
}

impl Update<Edificio> for EdificioDAO {
    fn update<C: DatabaseConnection>(conn: &C, item: Edificio) -> Result<Edificio, AppError> {
        let builder = QueryBuilder::update()
            .table(Self::table_name())
            .set("ANNO_COSTRUZIONE", item.anno_costruzione.clone())
            .set("ANNO_RIQUALIFICAZIONE", item.anno_riqualificazione.clone())
            .set("ISOLAMENTO_TETTO", item.isolamento_tetto)
            .set("CAPPOTTO", item.cappotto)
            .where_eq("CHIAVE", item.chiave.clone());

        let (query, param) = builder.build()?;

        match conn.execute(
            query.as_str(),
            rusqlite::params_from_iter(convert_param(param)),
        ) {
            Ok(_) => {
                info!("Edificio aggiornato con successo");
                Ok(item)
            }
            Err(e) => {
                error!("Errore durante l'aggiornamento {{ edificio }}: {}", e);
                Err(e)
            }
        }
    }
}
