use crate::dao::entity::Stanza;
use crate::database::DatabaseConnection;
use crate::dto::StanzaDto;
use itertools::Itertools;
use rusqlite::{params, Connection};

pub trait StanzaDao {
    fn get_all(conn: &Connection) -> Result<Vec<Stanza>, String>;
    fn insert<C: DatabaseConnection>(conn: &C, stanza: StanzaDto) -> Result<Stanza, String>;
    fn update<C: DatabaseConnection>(conn: &C, stanza: StanzaDto) -> Result<Stanza, String>;
    fn get_with_infissi(conn: &Connection, id: i64) -> Result<Stanza, String>;
}

pub struct StanzaDaoImpl;

impl StanzaDao for StanzaDaoImpl {
    fn get_all(conn: &Connection) -> Result<Vec<Stanza>, String> {
        let mut stmt = conn.prepare("SELECT * FROM STANZA").ok().unwrap();
        let result: Result<Vec<Stanza>, rusqlite::Error> = stmt
            .query_map([], |row| {
                Ok(Stanza {
                    id: row.get::<_, u64>(0)?,
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

    fn insert<C: DatabaseConnection>(conn: &C, stanza: StanzaDto) -> Result<Stanza, String> {
        let mut stmt = conn.prepare(
            "INSERT INTO STANZA(CHIAVE, PIANO, ID_SPAZIO, STANZA, DESTINAZIONE_USO, ALTEZZA, SPESSORE_MURO, RISCALDAMENTO, RAFFRESCAMENTO, ILLUMINAZIONE)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)"
        ).map_err(|e| e.to_string())?;
        stmt.execute(params![
            &stanza.chiave,
            &stanza.piano,
            &stanza.id_spazio,
            &stanza.stanza,
            &stanza.destinazione_uso,
            &stanza.altezza,
            &stanza.spessore_muro,
            &stanza.riscaldamento,
            &stanza.raffrescamento
        ])
        .map_err(|e| e.to_string())?;
        Ok(Stanza::from(&stanza))
    }

    fn update<C: DatabaseConnection>(conn: &C, stanza: StanzaDto) -> Result<Stanza, String> {
        // fixme: creare un builder delle query rispetto ai campi non null del dto
        conn.execute(
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
            ],
        )
        .map_err(|e| e.to_string())?;

        if let Some(mut infissi) = stanza.infissi.clone() {
            infissi.sort();

            let conteggio_infissi: Vec<(String, i32)> = infissi
                .into_iter()
                .chunk_by(|x| x.clone())
                .into_iter()
                .map(|(id, group)| (id, group.count() as i32))
                .collect();

            for (id_infisso, conteggio) in conteggio_infissi {
                conn.execute(
                    "INSERT INTO STANZA_CON_INFISSI(ID_STANZA, ID_INFISSO, NUM_INFISSI) \
                    VALUES (?1, ?2, ?3)",
                    params![stanza.id, id_infisso, conteggio],
                )
                .map_err(|e| e.to_string())?;
            }
        }

        Ok(Stanza::from(&stanza))
    }

    #[allow(dead_code, unused_variables)]
    fn get_with_infissi(conn: &Connection, id: i64) -> Result<Stanza, String> {
        todo!()
    }
}
