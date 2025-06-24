use crate::app_traits::{EntityTrait, FromRow, SqlParams, ToInsert, ToRetrieveAll, ToUpdate};
use crate::dto::FotovoltaicoDTO;
use crate::utils::ToList;
use rusqlite::{Error, Row};
use std::any::Any;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Fotovoltaico {
    pub(crate) id: u64,
    pub(crate) id_edificio: String,
    pub(crate) potenza: f32,
    pub(crate) proprietario: String,
}

impl Fotovoltaico {
    #[cfg(test)]
    pub fn new(id_edificio: &str, potenza: f32, proprietario: &str) -> Self {
        Self {
            id: 0,
            id_edificio: id_edificio.to_string(),
            potenza,
            proprietario: proprietario.to_string(),
        }
    }

    pub fn get_fields() -> Vec<String> {
        vec![
            "id".to_string(),
            "id_edificio".to_string(),
            "potenza".to_string(),
            "proprietario".to_string(),
        ]
    }
}

impl From<FotovoltaicoDTO> for Fotovoltaico {
    fn from(value: FotovoltaicoDTO) -> Self {
        Self {
            id: value.id.unwrap_or(0),
            id_edificio: value.id_edificio,
            potenza: value.potenza,
            proprietario: value.proprietario,
        }
    }
}

impl ToList for Fotovoltaico {
    fn to_list(&self) -> Vec<Box<dyn Any>> {
        vec![
            Box::new(self.id),
            Box::new(self.id_edificio.clone()),
            Box::new(self.potenza),
            Box::new(self.proprietario.clone()),
        ]
    }
}

impl FromRow for Fotovoltaico {
    fn from_row(row: &Row) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            id: row.get("ID")?,
            id_edificio: row.get("ID_EDIFICIO")?,
            potenza: row.get("POTENZA")?,
            proprietario: row.get("PROPRIETARIO")?,
        })
    }
}

impl EntityTrait for Fotovoltaico {
    type PrimaryKey = u64;

    #[inline]
    fn table_name() -> String {
        "FOTOVOLTAICO".to_string()
    }

    #[inline]
    fn sql_create_table() -> String {
        format!(
            "CREATE TABLE IF NOT EXISTS {}
                (
                    ID           INTEGER PRIMARY KEY AUTOINCREMENT,
                    ID_EDIFICIO  TEXT REFERENCES EDIFICIO (CHIAVE),
                    POTENZA      REAL NOT NULL CHECK ( POTENZA >= 0 ),
                    PROPRIETARIO TEXT NOT NULL
                ) STRICT;",
            Self::table_name()
        )
    }
}

impl ToRetrieveAll for Fotovoltaico {}

impl ToInsert for Fotovoltaico {
    #[inline]
    fn to_insert() -> String {
        format!(
            "INSERT INTO {} (ID_EDIFICIO, POTENZA, PROPRIETARIO) VALUES (?, ?, ?) RETURNING *;",
            Self::table_name()
        )
    }

    fn to_insert_params(&self) -> Vec<&dyn SqlParams> {
        vec![&self.id_edificio, &self.potenza, &self.proprietario]
    }
}

impl ToUpdate for Fotovoltaico {
    fn to_update() -> String {
        format!(
            "UPDATE {} SET POTENZA = ?, PROPRIETARIO = ? WHERE ID = ? RETURNING *;",
            Self::table_name()
        )
    }

    fn to_update_params(&self) -> Vec<Box<&dyn SqlParams>> {
        vec![
            Box::new(&self.potenza),
            Box::new(&self.proprietario),
            Box::new(&self.id),
        ]
    }
}
