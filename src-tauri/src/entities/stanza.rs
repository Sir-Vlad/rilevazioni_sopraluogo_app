use crate::app_traits::{EntityTrait, FromRow, SqlParams, ToInsert, ToRetrieveAll, ToUpdate};
use crate::dto::StanzaDTO;
use rusqlite::{Error, Row};

#[derive(Clone, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Stanza {
    pub(crate) id: Option<u64>,
    pub(crate) chiave: String,
    pub(crate) piano: String,
    pub(crate) id_spazio: String,
    pub(crate) cod_stanza: String,
    pub(crate) destinazione_uso: String,
    pub(crate) altezza: Option<u16>,
    pub(crate) spessore_muro: Option<u8>,
    pub(crate) riscaldamento: Option<String>,
    pub(crate) raffrescamento: Option<String>,
    pub(crate) illuminazione: Option<String>,
}

impl Default for Stanza {
    fn default() -> Self {
        Self {
            id: None,
            chiave: "".to_string(),
            piano: "".to_string(),
            id_spazio: "".to_string(),
            cod_stanza: "".to_string(),
            destinazione_uso: "".to_string(),
            altezza: None,
            spessore_muro: None,
            riscaldamento: None,
            raffrescamento: None,
            illuminazione: None,
        }
    }
}

impl Stanza {
    pub fn new(
        chiave: &str,
        piano: &str,
        id_spazio: &str,
        stanza: &str,
        destinazione_uso: &str,
    ) -> Self {
        Stanza {
            chiave: chiave.to_string(),
            piano: piano.to_string(),
            id_spazio: id_spazio.to_string(),
            cod_stanza: stanza.to_string(),
            destinazione_uso: destinazione_uso.to_string(),
            ..Self::default()
        }
    }
}

impl From<StanzaDTO> for Stanza {
    fn from(value: StanzaDTO) -> Self {
        Stanza {
            id: Some(value.id),
            chiave: value.chiave.clone(),
            piano: value.piano.clone(),
            id_spazio: value.id_spazio.clone(),
            cod_stanza: value.stanza.clone(),
            destinazione_uso: value.destinazione_uso.clone(),
            altezza: value.altezza,
            spessore_muro: value.spessore_muro,
            riscaldamento: value.riscaldamento.clone(),
            raffrescamento: value.raffrescamento.clone(),
            illuminazione: value.illuminazione.clone(),
        }
    }
}

impl FromRow for Stanza {
    fn from_row(row: &Row) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            id: row.get("ID")?,
            chiave: row.get("CHIAVE")?,
            piano: row.get("PIANO")?,
            id_spazio: row.get("ID_SPAZIO")?,
            cod_stanza: row.get("STANZA")?,
            destinazione_uso: row.get("DESTINAZIONE_USO")?,
            altezza: row.get("ALTEZZA")?,
            spessore_muro: row.get("SPESSORE_MURO")?,
            riscaldamento: row.get("RISCALDAMENTO")?,
            raffrescamento: row.get("RAFFRESCAMENTO")?,
            illuminazione: row.get("ILLUMINAZIONE")?,
        })
    }
}

impl EntityTrait for Stanza {
    type PrimaryKey = u64;

    fn table_name() -> String {
        "STANZA".to_string()
    }

    fn sql_create_table() -> String {
        format!("CREATE TABLE IF NOT EXISTS {}
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
            ) STRICT;", Self::table_name())
    }
}
impl ToRetrieveAll for Stanza {}
impl ToInsert for Stanza {
    fn to_insert() -> String {
        format!(
            "INSERT INTO {} (CHIAVE, PIANO, ID_SPAZIO, STANZA, DESTINAZIONE_USO, ALTEZZA, \
            SPESSORE_MURO, RISCALDAMENTO, RAFFRESCAMENTO, ILLUMINAZIONE) \
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING *;",
            Self::table_name()
        )
    }

    fn to_insert_params(&self) -> Vec<&dyn SqlParams> {
        vec![
            &self.chiave,
            &self.piano,
            &self.id_spazio,
            &self.cod_stanza,
            &self.destinazione_uso,
            &self.altezza,
            &self.spessore_muro,
            &self.riscaldamento,
            &self.raffrescamento,
            &self.illuminazione,
        ]
    }
}
impl ToUpdate for Stanza {
    fn to_update() -> String {
        format!(
            "UPDATE {} SET ALTEZZA = ?, SPESSORE_MURO = ?, RISCALDAMENTO = ?, RAFFRESCAMENTO = ?, ILLUMINAZIONE = ? WHERE ID = ? RETURNING *;",
            Self::table_name()
        )
    }

    fn to_update_params(&self) -> Vec<Box<&dyn SqlParams>> {
        vec![
            Box::new(&self.altezza),
            Box::new(&self.spessore_muro),
            Box::new(&self.riscaldamento),
            Box::new(&self.raffrescamento),
            Box::new(&self.illuminazione),
            Box::new(&self.id),
        ]
    }
}
