use crate::app_traits::{EntityTrait, FromRow, SqlParams, ToInsert, ToRetrieveAll};
use crate::dto::TipoDTO;
use rusqlite::{Error, Row};

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Illuminazione {
    pub(crate) _id: Option<u64>,
    pub(crate) lampadina: String,
    pub(crate) efficienza_energetica: u8,
}

impl Illuminazione {
    pub(crate) fn new(lampadina: String, efficienza_energetica: u8) -> Illuminazione {
        Self {
            _id: None,
            lampadina,
            efficienza_energetica,
        }
    }
}

impl From<TipoDTO> for Illuminazione {
    fn from(value: TipoDTO) -> Self {
        Self {
            _id: None,
            lampadina: value.name.clone(),
            efficienza_energetica: value.efficienza_energetica,
        }
    }
}

impl FromRow for Illuminazione {
    fn from_row(row: &Row) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            _id: row.get("ID")?,
            lampadina: row.get("LAMPADINA")?,
            efficienza_energetica: row.get("EFFICIENZA_ENERGETICA")?,
        })
    }
}

impl EntityTrait for Illuminazione {
    type PrimaryKey = u64;

    #[inline]
    fn table_name() -> String {
        "ILLUMINAZIONE".to_string()
    }

    #[inline]
    fn sql_create_table() -> String {
        format!(
            "CREATE TABLE IF NOT EXISTS {}
                    (
                        ID                    INTEGER PRIMARY KEY AUTOINCREMENT,
                        LAMPADINA             TEXT    NOT NULL UNIQUE COLLATE NOCASE,
                        EFFICIENZA_ENERGETICA INTEGER NOT NULL
                    ) STRICT;",
            Self::table_name()
        )
    }
}
impl ToRetrieveAll for Illuminazione {}
impl ToInsert for Illuminazione {
    #[inline]
    fn to_insert() -> String {
        format!(
            "INSERT INTO {} (LAMPADINA, EFFICIENZA_ENERGETICA) VALUES (?, ?) RETURNING *;",
            Self::table_name()
        )
    }

    fn to_insert_params(&self) -> Vec<&dyn SqlParams> {
        vec![&self.lampadina, &self.efficienza_energetica]
    }
}
