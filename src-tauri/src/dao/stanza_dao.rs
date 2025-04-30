use crate::dao::crud_operations::{GetAll, Insert, Update};
use crate::dao::entity::Stanza;
use crate::dao::utils::schema_operations::CreateTable;
use crate::database::WhereBuilder;
use crate::database::{convert_param, DatabaseConnection, QueryBuilder, SqlQueryBuilder};
use itertools::Itertools;
use log::{error, info};
use rusqlite::{params, Connection};
use std::collections::HashMap;

pub trait StanzaDAO {
    fn get_infissi_by_id<C: DatabaseConnection>(
        conn: &C,
        id_stanza: u64,
    ) -> Result<Vec<String>, String>;
    fn get_infissi_by_all(conn: &Connection) -> Result<HashMap<String, Vec<String>>, String>;
    fn set_infissi_by_id<C: DatabaseConnection>(
        conn: &C,
        id_stanza: u64,
        infissi: Vec<String>,
    ) -> Result<(), String>;
}

pub struct StanzaDAOImpl;

impl GetAll<Stanza> for StanzaDAOImpl {
    fn get_all<C: DatabaseConnection>(conn: &C) -> Result<Vec<Stanza>, String> {
        let query = match QueryBuilder::select().table("STANZA").build() {
            Ok((q, _)) => q,
            Err(e) => return Err(e.to_string()),
        };
        let mut stmt = conn.prepare(query.as_str()).map_err(|e| e.to_string())?;
        let result: Result<Vec<Stanza>, rusqlite::Error> = stmt
            .query_map([], |row| {
                Ok(Stanza {
                    id: row.get::<_, Option<u64>>("ID")?,
                    chiave: row.get::<_, String>("CHIAVE")?,
                    piano: row.get::<_, String>("PIANO")?,
                    id_spazio: row.get::<_, String>("ID_SPAZIO")?,
                    stanza: row.get::<_, String>("STANZA")?,
                    destinazione_uso: row.get::<_, String>("DESTINAZIONE_USO")?,
                    altezza: row.get::<_, Option<u16>>("ALTEZZA")?,
                    spessore_muro: row.get::<_, Option<u8>>("SPESSORE_MURO")?,
                    riscaldamento: row.get::<_, Option<String>>("RISCALDAMENTO")?,
                    raffrescamento: row.get::<_, Option<String>>("RAFFRESCAMENTO")?,
                    illuminazione: row.get::<_, Option<String>>("ILLUMINAZIONE")?,
                })
            })
            .map_err(|e| {
                format!(
                    "Errore nella lettura dei dati dal database: {:?}",
                    e.to_string()
                )
            })?
            .collect();

        match result {
            Ok(stanze) => Ok(stanze),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl Insert<Stanza> for StanzaDAOImpl {
    fn insert<C: DatabaseConnection>(conn: &C, stanza: Stanza) -> Result<Stanza, String> {
        let builder = QueryBuilder::insert()
            .table("STANZA")
            .columns(vec![
                "CHIAVE",
                "PIANO",
                "ID_SPAZIO",
                "STANZA",
                "DESTINAZIONE_USO",
                "ALTEZZA",
                "SPESSORE_MURO",
                "RISCALDAMENTO",
                "RAFFRESCAMENTO",
                "ILLUMINAZIONE",
            ])
            .values(vec![
                stanza.chiave.clone().into(),
                stanza.piano.clone().into(),
                stanza.id_spazio.clone().into(),
                stanza.stanza.clone().into(),
                stanza.destinazione_uso.clone().into(),
                stanza.altezza.into(),
                stanza.spessore_muro.into(),
                stanza.riscaldamento.clone().into(),
                stanza.raffrescamento.clone().into(),
                stanza.illuminazione.clone().into(),
            ]);
        let (query, param) = match builder.build() {
            Ok((q, p)) => (q, p),
            Err(e) => return Err(e.to_string()),
        };

        let mut stmt = conn.prepare(query.as_str()).map_err(|e| e.to_string())?;
        match stmt
            .execute(rusqlite::params_from_iter(convert_param(param)))
            .map_err(|e| e.to_string())
        {
            Ok(_) => {
                info!("Stanza inserita con successo");
                Ok(stanza)
            }
            Err(e) => {
                error!("Errore durante l'inserimento {{ stanza }}: {e}");
                Err(e.to_string())
            }
        }
    }
}

impl Update<Stanza> for StanzaDAOImpl {
    fn update<C: DatabaseConnection>(conn: &C, stanza: Stanza) -> Result<Stanza, String> {
        let builder = QueryBuilder::update()
            .table("STANZA")
            .set_if("ALTEZZA", stanza.altezza)
            .set_if("SPESSORE_MURO", stanza.spessore_muro)
            .set_if("RISCALDAMENTO", stanza.riscaldamento.clone())
            .set_if("RAFFRESCAMENTO", stanza.raffrescamento.clone())
            .set_if("ILLUMINAZIONE", stanza.illuminazione.clone())
            .where_eq("ID", stanza.id.unwrap());
        let (query, param) = match builder.build() {
            Ok((q, p)) => (q, p),
            Err(e) => return Err(e.to_string()),
        };

        match conn.execute(
            query.as_str(),
            rusqlite::params_from_iter(convert_param(param)),
        ) {
            Ok(_) => {
                info!("Stanza aggiornata con successo");
                Ok(stanza)
            }
            Err(e) => {
                error!("Errore durante l'aggiornamento {{ stanza }}: {e}");
                Err(e.to_string())
            }
        }
    }
}

impl StanzaDAO for StanzaDAOImpl {
    fn get_infissi_by_id<C: DatabaseConnection>(conn: &C, id: u64) -> Result<Vec<String>, String> {
        let builder = QueryBuilder::select()
            .table("STANZA_CON_INFISSI")
            .where_eq("ID_STANZA", id);
        let query = match builder.build() {
            Ok((q, _p)) => q,
            Err(e) => return Err(e.to_string()),
        };
        let mut stmt = conn.prepare(query.as_str()).map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map(params![id], |row| {
                let id_infisso = row.get::<_, String>("ID_INFISSO")?;
                let ripetizioni = row.get::<_, u16>("NUM_INFISSI")?;
                Ok((id_infisso, ripetizioni))
            })
            .map_err(|e| e.to_string())?;

        let mut infissi: Vec<String> = Vec::new();
        for (id_infisso, ripetizioni) in rows.flatten() {
            for _ in 0..ripetizioni {
                infissi.push(id_infisso.clone());
            }
        }

        Ok(infissi)
    }

    fn get_infissi_by_all(conn: &Connection) -> Result<HashMap<String, Vec<String>>, String> {
        let (query, _) = QueryBuilder::select()
            .table("STANZA_CON_INFISSI")
            .build()
            .map_err(|e| e.to_string())?;
        let mut stmt = conn.prepare(query.as_str()).map_err(|e| e.to_string())?;

        let mut infissi: HashMap<String, Vec<String>> = HashMap::new();
        let mut rows = stmt.query([]).map_err(|e| e.to_string())?;

        while let Some(row) = rows.next().map_err(|e| e.to_string())? {
            let id_stanza = row.get::<_, i64>("ID_STANZA").map_err(|e| e.to_string())?;
            let id_infisso = row
                .get::<_, String>("ID_INFISSO")
                .map_err(|e| e.to_string())?;
            let num_infissi = row
                .get::<_, i32>("NUM_INFISSI")
                .map_err(|e| e.to_string())?;

            let stanza_infissi = infissi
                .entry(id_stanza.to_string())
                .or_insert_with(Vec::new);

            for _ in 0..num_infissi {
                stanza_infissi.push(id_infisso.clone());
            }
        }

        Ok(infissi)
    }

    fn set_infissi_by_id<C: DatabaseConnection>(
        conn: &C,
        id: u64,
        infissi: Vec<String>,
    ) -> Result<(), String> {
        // fixme: se esiste già l'infisso bisogna solo incrementare il numero di infissi
        let infissi_exists = StanzaDAOImpl::get_infissi_by_id(conn, id)?.is_empty();
        if infissi_exists {
            return Err(format!("Database stanza id {} not exist", id));
        }

        let mut infissi = infissi;
        infissi.sort();

        let conteggio_infissi: Vec<(String, i32)> = infissi
            .into_iter()
            .chunk_by(|x| x.clone())
            .into_iter()
            .map(|(id, group)| (id, group.count() as i32))
            .collect();

        let builder = QueryBuilder::insert()
            .table("STANZA_CON_INFISSI")
            .columns(vec!["ID_STANZA", "ID_INFISSO", "NUM_INFISSI"])
            .values(vec![0.into(), "A".into(), 0.into()]); // param fake
        let query = match builder.build() {
            Ok((q, _p)) => q,
            Err(e) => return Err(e.to_string()),
        };

        for (id_infisso, conteggio) in conteggio_infissi {
            match conn.execute(query.as_str(), params![id, id_infisso, conteggio]) {
                Ok(_) => info!("Stanze_con_infissi inserito con successo"),
                Err(e) => {
                    error!("Errore durante l'inserimento {{ stanze_con_infissi }}: {e}");
                    return Err(e.to_string());
                }
            }
        }

        Ok(())
    }
}

impl CreateTable for StanzaDAOImpl {
    fn create_table<C: DatabaseConnection>(conn: &C) -> Result<(), String> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS STANZA
            (
                ID               INTEGER PRIMARY KEY AUTOINCREMENT,
                CHIAVE           TEXT NOT NULL REFERENCES EDIFICIO (CHIAVE),
                PIANO            TEXT NOT NULL,
                ID_SPAZIO        TEXT NOT NULL,
                STANZA           TEXT NOT NULL,
                DESTINAZIONE_USO TEXT NOT NULL,
                ALTEZZA          INTEGER CHECK ( ALTEZZA >= 0 )       DEFAULT 0,
                SPESSORE_MURO    INTEGER CHECK ( SPESSORE_MURO >= 0 ) DEFAULT 0,
                RISCALDAMENTO    TEXT                                 DEFAULT NULL REFERENCES CLIMATIZZAZIONE (CLIMATIZZAZIONE),
                RAFFRESCAMENTO   TEXT                                 DEFAULT NULL REFERENCES CLIMATIZZAZIONE (CLIMATIZZAZIONE),
                ILLUMINAZIONE    TEXT                                 DEFAULT NULL REFERENCES ILLUMINAZIONE (LAMPADINA),
                UNIQUE (CHIAVE, ID_SPAZIO, STANZA, DESTINAZIONE_USO)
            ) STRICT;",
            ()).map_err(|e| e.to_string())?;
        info!("Tabella STANZA creata");

        conn.execute(
            "CREATE TABLE IF NOT EXISTS STANZA_CON_INFISSI
            (
                ID_STANZA   INTEGER NOT NULL REFERENCES STANZA (ID),
                ID_INFISSO  TEXT    NOT NULL REFERENCES INFISSO (ID),
                NUM_INFISSI INTEGER NOT NULL DEFAULT 1 CHECK ( NUM_INFISSI > 0 ),
                PRIMARY KEY (ID_INFISSO, ID_STANZA)
            ) STRICT;",
            (),
        )
        .map_err(|e| e.to_string())?;
        info!("Tabella STANZA_CON_INFISSI creata");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::open_in_memory().unwrap();
        Ok(())
    }

    #[test]
    fn get_infissi_by_id() {}
}
