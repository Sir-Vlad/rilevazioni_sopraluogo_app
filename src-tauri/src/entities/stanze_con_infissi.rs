use crate::app_traits::{
    EntityTrait, FromRow, SqlParams, ToInsert, ToRetrieve, ToRetrieveAll, ToUpdate,
};
use rusqlite::{Error, Row};
use std::collections::HashMap;

#[cfg_attr(test, derive(PartialEq, Debug, Clone))]
pub struct StanzaConInfissi {
    pub(crate) id_stanza: u64,
    pub(crate) id_infissi: Vec<(String, u64)>,
    pub(crate) id_edificio: String,
}

impl StanzaConInfissi {
    pub fn new(id_stanza: u64, id_infissi: Vec<(String, u64)>, id_edificio: String) -> Self {
        Self {
            id_stanza,
            id_infissi,
            id_edificio,
        }
    }

    pub fn new_with_infissi_expanse(
        id_stanza: u64,
        id_infissi: Vec<String>,
        id_edificio: String,
    ) -> Self {
        let mut conteggio = HashMap::new();
        for infissi in id_infissi {
            *conteggio.entry(infissi).or_insert(0) += 1;
        }

        Self::new(
            id_stanza,
            conteggio
                .into_iter()
                .map(|(id, count)| (id, count))
                .collect(),
            id_edificio,
        )
    }

    pub fn expanse_infissi(&self) -> Vec<String> {
        self.id_infissi
            .iter()
            .flat_map(|(id, count)| std::iter::repeat(id.to_string()).take(*count as usize))
            .collect()
    }
}

impl FromRow for StanzaConInfissi {
    fn from_row(row: &Row) -> Result<Self, Error>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl EntityTrait for StanzaConInfissi {
    type PrimaryKey = (u64, String);

    fn table_name() -> String {
        "STANZA_CON_INFISSI".to_string()
    }

    fn sql_create_table() -> String {
        format!(
            "CREATE TABLE IF NOT EXISTS {}
                (
                    ID_STANZA      INTEGER NOT NULL REFERENCES STANZA (ID),
                    ID_INFISSO     TEXT    NOT NULL,
                    ID_EDIFICIO    TEXT    NOT NULL,
                    NUM_INFISSI    INTEGER NOT NULL DEFAULT 1 CHECK ( NUM_INFISSI > 0 ),
                    PRIMARY KEY (ID_INFISSO, ID_STANZA, ID_EDIFICIO),
                    FOREIGN KEY (ID_INFISSO, ID_EDIFICIO) REFERENCES INFISSO (ID, EDIFICIO)
                ) STRICT;",
            Self::table_name()
        )
    }
}
impl ToRetrieve for StanzaConInfissi {
    fn to_retrieve() -> String {
        format!(
            "SELECT * FROM {} WHERE ID_STANZA = ? AND ID_EDIFICIO = ?;",
            Self::table_name()
        )
    }
}
impl ToRetrieveAll for StanzaConInfissi {}
impl ToInsert for StanzaConInfissi {
    fn to_insert() -> String {
        format!(
            "INSERT INTO {}(ID_STANZA, ID_INFISSO, ID_EDIFICIO, NUM_INFISSI) VALUES (?,?,?,?)",
            Self::table_name()
        )
    }

    fn to_insert_params(&self) -> Vec<&dyn SqlParams> {
        panic!("This method should not be called")
    }
}
impl ToUpdate for StanzaConInfissi {
    fn to_update() -> String {
        format!(
            "UPDATE {} SET NUM_INFISSI = ? WHERE ID_INFISSO = ? AND ID_STANZA = ? AND ID_EDIFICIO = ?;",
            Self::table_name()
        )
    }

    fn to_update_params(&self) -> Vec<Box<&dyn SqlParams>> {
        panic!("This method should not be called")
    }
}
