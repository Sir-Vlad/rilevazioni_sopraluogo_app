use crate::app_traits::{EntityTrait, FromRow, SqlParams, ToInsert, ToRetrieveAll};
use rusqlite::{Error, Row};

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct MaterialeInfisso {
    pub(crate) _id: Option<u64>,
    pub(crate) materiale: String,
    pub(crate) efficienza_energetica: u8,
}

impl MaterialeInfisso {
    pub(crate) fn new(materiale: String, efficienza_energetica: u8) -> Self {
        Self {
            _id: None,
            materiale,
            efficienza_energetica,
        }
    }
}

impl FromRow for MaterialeInfisso {
    fn from_row(row: &Row) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            _id: row.get("ID")?,
            materiale: row.get("MATERIALE")?,
            efficienza_energetica: row.get("EFFICIENZA_ENERGETICA")?,
        })
    }
}

impl EntityTrait for MaterialeInfisso {
    type PrimaryKey = u64;

    #[inline]
    fn table_name() -> String {
        "MATERIALE_INFISSO".to_string()
    }

    #[inline]
    fn sql_create_table() -> String {
        format!(
            "CREATE TABLE IF NOT EXISTS {}
                (
                    ID                    INTEGER PRIMARY KEY AUTOINCREMENT,
                    MATERIALE             TEXT    NOT NULL UNIQUE COLLATE NOCASE,
                    EFFICIENZA_ENERGETICA INTEGER NOT NULL
                ) STRICT;",
            Self::table_name()
        )
    }
}
impl ToRetrieveAll for MaterialeInfisso {}
impl ToInsert for MaterialeInfisso {
    #[inline]
    fn to_insert() -> String {
        format!(
            "INSERT INTO {} (MATERIALE, EFFICIENZA_ENERGETICA) VALUES (?, ?) RETURNING *;",
            Self::table_name()
        )
    }

    fn to_insert_params(&self) -> Vec<&dyn SqlParams> {
        vec![&self.materiale, &self.efficienza_energetica]
    }
}
