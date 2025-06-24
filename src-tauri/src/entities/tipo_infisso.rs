use crate::app_traits::{EntityTrait, FromRow, SqlParams, ToInsert, ToRetrieveAll};
use rusqlite::{Error, Row};

pub struct TipoInfisso {
    pub(crate) _id: u64,
    pub(crate) nome: String,
}

impl FromRow for TipoInfisso {
    fn from_row(row: &Row) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            _id: row.get("ID")?,
            nome: row.get("NOME")?,
        })
    }
}

impl EntityTrait for TipoInfisso {
    type PrimaryKey = u64;

    fn table_name() -> String {
        "TIPO_INFISSO".to_string()
    }

    fn sql_create_table() -> String {
        format!(
            "CREATE TABLE IF NOT EXISTS {}
                (
                    ID   INTEGER PRIMARY KEY,
                    NOME TEXT NOT NULL UNIQUE
                )
                ",
            Self::table_name()
        )
    }
}

impl ToRetrieveAll for TipoInfisso {}

impl ToInsert for TipoInfisso {
    fn to_insert() -> String {
        format!(
            "INSERT INTO {} (NOME) VALUES (?) RETURNING *",
            Self::table_name()
        )
    }

    fn to_insert_params(&self) -> Vec<&dyn SqlParams> {
        vec![&self.nome]
    }
}
