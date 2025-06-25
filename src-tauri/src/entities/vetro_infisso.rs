use crate::app_traits::{EntityTrait, FromRow, SqlParams, ToInsert, ToRetrieveAll};
use rusqlite::{Error, Row};

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct VetroInfisso {
    pub(crate) _id: Option<u64>,
    pub(crate) vetro: String,
    pub(crate) efficienza_energetica: u8,
}

impl VetroInfisso {
    #[cfg(test)]
    pub(crate) fn new(vetro: &str, efficienza_energetica: u8) -> Self {
        Self {
            _id: None,
            vetro: vetro.to_string(),
            efficienza_energetica,
        }
    }
}

impl FromRow for VetroInfisso {
    fn from_row(row: &Row) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            _id: row.get("ID")?,
            vetro: row.get("VETRO")?,
            efficienza_energetica: row.get("EFFICIENZA_ENERGETICA")?,
        })
    }
}

impl EntityTrait for VetroInfisso {
    type PrimaryKey = u64;

    #[inline]
    fn table_name() -> String {
        "VETRO_INFISSO".to_string()
    }

    #[inline]
    fn sql_create_table() -> String {
        format!(
            "CREATE TABLE IF NOT EXISTS {}
                (
                    ID                    INTEGER PRIMARY KEY AUTOINCREMENT,
                    VETRO                 TEXT    NOT NULL UNIQUE COLLATE NOCASE,
                    EFFICIENZA_ENERGETICA INTEGER NOT NULL
                ) STRICT;",
            Self::table_name()
        )
    }
}
impl ToRetrieveAll for VetroInfisso {}
impl ToInsert for VetroInfisso {
    #[inline]
    fn to_insert() -> String {
        format!(
            "INSERT INTO {} (VETRO, EFFICIENZA_ENERGETICA) VALUES (?, ?) RETURNING *;",
            Self::table_name()
        )
    }

    fn to_insert_params(&self) -> Vec<&dyn SqlParams> {
        vec![&self.vetro, &self.efficienza_energetica]
    }
}
