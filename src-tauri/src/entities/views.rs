use crate::app_traits::{EntityTrait, FromRow, ToRetrieveAll};
use crate::utils::ToList;
use rusqlite::{Error, Row};
use serde::Serialize;
use std::any::Any;

#[derive(Debug, Serialize)]
pub struct DatiStanza {
    pub(crate) id: u64,
    pub(crate) fascicolo: String,
    pub(crate) chiave: String,
    pub(crate) piano: String,
    pub(crate) id_spazio: String,
    pub(crate) stanza: String,
    pub(crate) destinazione_uso: String,
    pub(crate) altezza: Option<u16>,
    pub(crate) spessore_muro: Option<u8>,
    pub(crate) riscaldamento: Option<String>,
    pub(crate) raffrescamento: Option<String>,
    pub(crate) illuminazione: Option<String>,
    pub(crate) mq_infissi: Option<f32>,
    pub(crate) materiale: Option<String>,
    pub(crate) vetro: Option<String>,
}

impl DatiStanza {
    pub fn get_fields() -> Vec<&'static str> {
        vec![
            "id",
            "fascicolo",
            "chiave",
            "piano",
            "id_spazio",
            "stanza",
            "destinazione_uso",
            "altezza",
            "spessore_muro",
            "riscaldamento",
            "raffrescamento",
            "illuminazione",
            "mq_infissi",
            "materiale",
            "vetro",
        ]
    }
}

impl ToList for DatiStanza {
    fn to_list(&self) -> Vec<Box<dyn Any>> {
        vec![
            Box::new(self.id),
            Box::new(self.fascicolo.clone()),
            Box::new(self.chiave.clone()),
            Box::new(self.piano.clone()),
            Box::new(self.id_spazio.clone()),
            Box::new(self.stanza.clone()),
            Box::new(self.destinazione_uso.clone()),
            Box::new(self.altezza),
            Box::new(self.spessore_muro),
            Box::new(self.riscaldamento.clone()),
            Box::new(self.raffrescamento.clone()),
            Box::new(self.illuminazione.clone()),
            Box::new(self.mq_infissi),
            Box::new(self.materiale.clone()),
            Box::new(self.vetro.clone()),
        ]
    }
}

impl FromRow for DatiStanza {
    fn from_row(row: &Row) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(DatiStanza {
            id: row.get("ID")?,
            fascicolo: row.get("FASCICOLO")?,
            chiave: row.get("CHIAVE")?,
            piano: row.get("PIANO")?,
            id_spazio: row.get("ID_SPAZIO")?,
            stanza: row.get("STANZA")?,
            destinazione_uso: row.get("DESTINAZIONE_USO")?,
            altezza: row.get("ALTEZZA")?,
            spessore_muro: row.get("SPESSORE_MURO")?,
            riscaldamento: row.get("RISCALDAMENTO")?,
            raffrescamento: row.get("RAFFRESCAMENTO")?,
            illuminazione: row.get("ILLUMINAZIONE")?,
            mq_infissi: row.get("MQ_INFISSI")?,
            materiale: row.get("MATERIALE")?,
            vetro: row.get("VETRO")?,
        })
    }
}

impl EntityTrait for DatiStanza {
    type PrimaryKey = u64;

    fn table_name() -> String {
        "V_DATI_STANZE".to_string()
    }

    fn sql_create_table() -> String {
        format!(
            "CREATE VIEW IF NOT EXISTS {} AS
                SELECT S.ID, E.FASCICOLO, S.CHIAVE, S.PIANO, S.ID_SPAZIO, 
                    S.STANZA, S.DESTINAZIONE_USO,
                    S.ALTEZZA, S.SPESSORE_MURO,
                    S.RISCALDAMENTO, S.RAFFRESCAMENTO, S.ILLUMINAZIONE, 
                    coalesce(round(DGS.MQ_INFISSI, 2), 0) AS MQ_INFISSI,
                    DGS.MATERIALE, DGS.VETRO
                FROM STANZA AS S
                     JOIN EDIFICIO AS E ON E.CHIAVE = S.CHIAVE
                     -- Aggiungo i dati degli infissi alle stanze che c'è li hanno
                     LEFT JOIN (
                       -- Recupero i dati delle stanze che hanno degli infissi
                       SELECT MQ.ID_STANZA, MQ.MQ_INFISSI, M.MATERIALE, V.VETRO
                       FROM {} AS MQ
                                JOIN {} AS M 
                                    ON MQ.ID_STANZA = M.ID_STANZA
                                JOIN {} AS V 
                                    ON M.ID_STANZA = v.ID_STANZA
                       ) AS DGS ON S.ID = DGS.ID_STANZA;",
            Self::table_name(),
            MqInfissi::table_name(),
            MatMinEffStanza::table_name(),
            VetMinEffStanza::table_name(),
        )
    }
}

impl ToRetrieveAll for DatiStanza {}

pub struct MatMinEffStanza;

impl FromRow for MatMinEffStanza {
    fn from_row(row: &Row) -> Result<Self, Error>
    where
        Self: Sized,
    {
        panic!("Should not be called")
    }
}

impl EntityTrait for MatMinEffStanza {
    type PrimaryKey = u64;

    fn table_name() -> String {
        "V_MAT_MIN_EFF_STANZA".to_string()
    }

    fn sql_create_table() -> String {
        format!(
            "CREATE VIEW IF NOT EXISTS {} AS
                SELECT SI.ID_STANZA, MI.MATERIALE
                FROM STANZA AS S
                     JOIN STANZA_CON_INFISSI AS SI ON S.ID = SI.ID_STANZA
                     JOIN INFISSO AS I ON SI.ID_INFISSO = I.ID
                     JOIN MATERIALE_INFISSO AS MI ON I.MATERIALE = MI.MATERIALE
                WHERE MI.EFFICIENZA_ENERGETICA IN (
                      SELECT MIN(MI2.EFFICIENZA_ENERGETICA)
                      FROM STANZA_CON_INFISSI AS SI
                               JOIN INFISSO AS I2 ON SI.ID_INFISSO = I2.ID
                               JOIN MATERIALE_INFISSO AS MI2 ON I2.MATERIALE = MI2.MATERIALE
                      WHERE SI.ID_STANZA = S.ID)
                GROUP BY SI.ID_STANZA, MI.MATERIALE;",
            Self::table_name()
        )
    }
}

pub struct VetMinEffStanza;

impl FromRow for VetMinEffStanza {
    fn from_row(row: &Row) -> Result<Self, Error>
    where
        Self: Sized,
    {
        panic!("Should not be called")
    }
}

impl EntityTrait for VetMinEffStanza {
    type PrimaryKey = u64;

    fn table_name() -> String {
        "V_VET_MIN_EFF_STANZA".to_string()
    }

    fn sql_create_table() -> String {
        format!(
            "CREATE VIEW IF NOT EXISTS {} AS
                SELECT SI.ID_STANZA, MI.VETRO
                FROM STANZA AS S
                         JOIN STANZA_CON_INFISSI AS SI ON S.ID = SI.ID_STANZA
                         JOIN INFISSO AS I ON SI.ID_INFISSO = I.ID
                         JOIN VETRO_INFISSO AS MI ON I.VETRO = MI.VETRO
                WHERE MI.EFFICIENZA_ENERGETICA IN (
                          SELECT MIN(MI2.EFFICIENZA_ENERGETICA)
                          FROM STANZA_CON_INFISSI AS SI
                                   JOIN INFISSO AS I2 ON SI.ID_INFISSO = I2.ID
                                   JOIN VETRO_INFISSO AS MI2 ON I2.VETRO = MI2.VETRO
                          WHERE SI.ID_STANZA = S.ID)
                GROUP BY SI.ID_STANZA, MI.VETRO;",
            Self::table_name()
        )
    }
}

pub struct MqInfissi;

impl FromRow for MqInfissi {
    fn from_row(row: &Row) -> Result<Self, Error>
    where
        Self: Sized,
    {
        panic!("Should not be called");
    }
}

impl EntityTrait for MqInfissi {
    type PrimaryKey = u64;

    fn table_name() -> String {
        "V_MQ_INFISSI".to_string()
    }

    fn sql_create_table() -> String {
        format!(
            "CREATE VIEW IF NOT EXISTS {} AS
                SELECT SI.ID_STANZA, SUM(I.MQ * SI.NUM_INFISSI) AS 'MQ_INFISSI'
                FROM INFISSO AS I
                         JOIN STANZA_CON_INFISSI AS SI ON I.ID = SI.ID_INFISSO
                GROUP BY SI.ID_STANZA;",
            Self::table_name()
        )
    }
}
