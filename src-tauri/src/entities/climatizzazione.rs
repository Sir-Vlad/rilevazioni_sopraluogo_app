use crate::app_traits::{EntityTrait, FromRow, SqlParams, ToInsert, ToRetrieveAll};
use crate::dto::TipoDTO;
use rusqlite::{Error, Row};

#[cfg_attr(test, derive(Debug, PartialEq, Clone))]
pub struct Climatizzazione {
    pub(crate) _id: Option<u64>,
    pub(crate) climatizzazione: String,
    pub(crate) efficienza_energetica: u8,
}

impl Climatizzazione {
    pub(crate) fn new(climatizzazione: String, efficienza_energetica: u8) -> Climatizzazione {
        Self {
            _id: None,
            climatizzazione,
            efficienza_energetica,
        }
    }
}

impl From<TipoDTO> for Climatizzazione {
    fn from(value: TipoDTO) -> Self {
        Self {
            _id: None,
            climatizzazione: value.name.clone(),
            efficienza_energetica: value.efficienza_energetica,
        }
    }
}

impl FromRow for Climatizzazione {
    fn from_row(row: &Row) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            _id: row.get("ID")?,
            climatizzazione: row.get("CLIMATIZZAZIONE")?,
            efficienza_energetica: row.get("EFFICIENZA_ENERGETICA")?,
        })
    }
}

impl EntityTrait for Climatizzazione {
    type PrimaryKey = u64;

    #[inline]
    fn table_name() -> String {
        "CLIMATIZZAZIONE".to_string()
    }

    #[inline]
    fn sql_create_table() -> String {
        format!(
            "CREATE TABLE IF NOT EXISTS {}
                (
                    ID                    INTEGER PRIMARY KEY AUTOINCREMENT,
                    CLIMATIZZAZIONE       TEXT    NOT NULL UNIQUE COLLATE NOCASE,
                    EFFICIENZA_ENERGETICA INTEGER NOT NULL
                ) STRICT;",
            Self::table_name()
        )
    }
}

impl ToRetrieveAll for Climatizzazione {}

impl ToInsert for Climatizzazione {
    #[inline]
    fn to_insert() -> String {
        format!(
            "INSERT INTO {} (CLIMATIZZAZIONE, EFFICIENZA_ENERGETICA) VALUES (?, ?) RETURNING *;",
            Self::table_name()
        )
    }

    fn to_insert_params(&self) -> Vec<&dyn SqlParams> {
        vec![&self.climatizzazione, &self.efficienza_energetica]
    }
}
