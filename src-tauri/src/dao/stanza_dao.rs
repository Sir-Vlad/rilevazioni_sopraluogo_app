use crate::dao::entity::Stanza;
use crate::database::DatabaseConnection;
use itertools::Itertools;
use log::{ error, info };
use rusqlite::{ params, Connection };
use std::collections::HashMap;

pub trait StanzaDao {
    fn get_all(conn: &Connection) -> Result<Vec<Stanza>, String>;
    fn insert<C: DatabaseConnection>(conn: &C, stanza: Stanza) -> Result<Stanza, String>;
    fn update<C: DatabaseConnection>(conn: &C, stanza: Stanza) -> Result<Stanza, String>;
    fn get_infissi_by_id(conn: &Connection, id_stanza: i64) -> Result<Vec<String>, String>;
    fn get_infissi_by_all(conn: &Connection) -> Result<HashMap<String, Vec<String>>, String>;
    fn set_infissi_by_id<C: DatabaseConnection>(
        conn: &C,
        id_stanza: u64,
        infissi: Vec<String>
    ) -> Result<(), String>;
}

pub struct StanzaDaoImpl;

impl StanzaDao for StanzaDaoImpl {
    fn get_all(conn: &Connection) -> Result<Vec<Stanza>, String> {
        let mut stmt = conn.prepare("SELECT * FROM STANZA").ok().unwrap();
        let result: Result<Vec<Stanza>, rusqlite::Error> = stmt
            .query_map([], |row| {
                Ok(Stanza {
                    id: row.get::<_, Option<u64>>(0)?,
                    chiave: row.get::<_, String>(1)?,
                    piano: row.get::<_, String>(2)?,
                    id_spazio: row.get::<_, String>(3)?,
                    stanza: row.get::<_, String>(4)?,
                    destinazione_uso: row.get::<_, String>(5)?,
                    altezza: row.get::<_, Option<u16>>(6)?,
                    spessore_muro: row.get::<_, Option<u8>>(7)?,
                    riscaldamento: row.get::<_, Option<String>>(8)?,
                    raffrescamento: row.get::<_, Option<String>>(9)?,
                    illuminazione: row.get::<_, Option<String>>(10)?,
                })
            })
            .map_err(|e| {
                format!("Errore nella lettura dei dati dal database: {:?}", e.to_string())
            })?
            .collect();

        match result {
            Ok(stanze) => Ok(stanze),
            Err(e) => Err(e.to_string()),
        }
    }

    fn insert<C: DatabaseConnection>(conn: &C, stanza: Stanza) -> Result<Stanza, String> {
        let mut stmt = conn
            .prepare(
                "INSERT INTO STANZA(CHIAVE, PIANO, ID_SPAZIO, STANZA, DESTINAZIONE_USO, ALTEZZA, SPESSORE_MURO, RISCALDAMENTO, RAFFRESCAMENTO, ILLUMINAZIONE)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)"
            )
            .map_err(|e| e.to_string())?;
        match
            stmt
                .execute(
                    params![
                        &stanza.chiave,
                        &stanza.piano,
                        &stanza.id_spazio,
                        &stanza.stanza,
                        &stanza.destinazione_uso,
                        &stanza.altezza,
                        &stanza.spessore_muro,
                        &stanza.riscaldamento,
                        &stanza.raffrescamento,
                        &stanza.illuminazione
                    ]
                )
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

    fn update<C: DatabaseConnection>(conn: &C, stanza: Stanza) -> Result<Stanza, String> {
        match
            conn
                .execute(
                    "
        UPDATE STANZA 
        SET ALTEZZA = ?1, 
            SPESSORE_MURO = ?2, 
            RISCALDAMENTO = ?3, 
            RAFFRESCAMENTO = ?4, 
            ILLUMINAZIONE = ?5
        WHERE ID = ?6
        ",
                    params![
                        stanza.altezza,
                        stanza.spessore_muro,
                        stanza.riscaldamento,
                        stanza.raffrescamento,
                        stanza.illuminazione,
                        stanza.id
                    ]
                )
                .map_err(|e| e.to_string())
        {
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
    
    fn get_infissi_by_id(conn: &Connection, id: i64) -> Result<Vec<String>, String> {
        let mut stmt = conn
            .prepare(
                "
                    SELECT * FROM STANZA_CON_INFISSI WHERE ID_STANZA = ?1
                    "
            )
            .map_err(|e| e.to_string())?;

        let rows = stmt
            .query_map(params![id], |row| {
                let id_infisso = row.get::<_, String>(1)?;
                let ripetizioni = row.get::<_, u16>(2)?;
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
        let mut stmt = conn
            .prepare("
                    SELECT * FROM STANZA_CON_INFISSI
                ")
            .map_err(|e| e.to_string())?;

        let mut infissi: HashMap<String, Vec<String>> = HashMap::new();
        let mut rows = stmt.query([]).map_err(|e| e.to_string())?;

        while let Some(row) = rows.next().map_err(|e| e.to_string())? {
            let id_stanza = row.get::<_, i64>(0).map_err(|e| e.to_string())?;
            let id_infisso = row.get::<_, String>(1).map_err(|e| e.to_string())?;
            let num_infissi = row.get::<_, i32>(2).map_err(|e| e.to_string())?;

            let stanza_infissi = infissi.entry(id_stanza.to_string()).or_insert_with(Vec::new);

            for _ in 0..num_infissi {
                stanza_infissi.push(id_infisso.clone());
            }
        }

        Ok(infissi)
    }

    fn set_infissi_by_id<C: DatabaseConnection>(
        conn: &C,
        id: u64,
        infissi: Vec<String>
    ) -> Result<(), String> {
        let mut infissi = infissi;
        infissi.sort();

        let conteggio_infissi: Vec<(String, i32)> = infissi
            .into_iter()
            .chunk_by(|x| x.clone())
            .into_iter()
            .map(|(id, group)| (id, group.count() as i32))
            .collect();

        for (id_infisso, conteggio) in conteggio_infissi {
            match
                conn.execute(
                    "INSERT INTO STANZA_CON_INFISSI(ID_STANZA, ID_INFISSO, NUM_INFISSI) \
                    VALUES (?1, ?2, ?3)",
                    params![id, id_infisso, conteggio]
                )
            {
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
