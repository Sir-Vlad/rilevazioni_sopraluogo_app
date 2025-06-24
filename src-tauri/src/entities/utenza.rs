use crate::app_traits::{EntityTrait, FromRow, SqlParams, ToInsert, ToRetrieveAll, ToUpdate};
use crate::database::QueryBuilderError;
use crate::dto::UtenzaDTO;
use rusqlite::{Error, Row};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Utenza {
    pub(crate) id: u64,
    pub(crate) id_edificio: String,
    pub(crate) tipo: TipoUtenza,
    pub(crate) cod_contatore: String,
    pub(crate) indirizzo_contatore: Option<String>,
    tipo_cached: Option<String>,
}

impl Utenza {
    #[cfg(test)]
    pub fn new(
        id_edificio: &str,
        tipo: &str,
        cod_contatore: &str,
        indirizzo_contatore: &str,
    ) -> Self {
        Self {
            id: 0,
            id_edificio: id_edificio.to_string(),
            tipo: tipo.into(),
            cod_contatore: cod_contatore.to_string(),
            indirizzo_contatore: Some(indirizzo_contatore.to_string()),
            tipo_cached: Some(tipo.into()),
        }
    }
}

impl From<UtenzaDTO> for Utenza {
    fn from(value: UtenzaDTO) -> Self {
        Self {
            id: value.id.unwrap_or(0),
            id_edificio: value.id_edificio,
            tipo: value.tipo.clone(),
            cod_contatore: value.cod_contatore,
            indirizzo_contatore: value.indirizzo_contatore,
            tipo_cached: Some(value.tipo.to_string()),
        }
    }
}

impl FromRow for Utenza {
    fn from_row(row: &Row) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let tipo_str = row.get::<_, String>("TIPO")?;

        Ok(Self {
            id: row.get("ID")?,
            id_edificio: row.get("ID_EDIFICIO")?,
            tipo: tipo_str.clone().into(),
            cod_contatore: row.get("COD_CONTATORE")?,
            indirizzo_contatore: row.get("INDIRIZZO_CONTATORE")?,
            tipo_cached: Some(tipo_str),
        })
    }
}

impl EntityTrait for Utenza {
    type PrimaryKey = u64;

    fn table_name() -> String {
        "UTENZE".to_string()
    }

    fn sql_create_table() -> String {
        format!(
            "CREATE TABLE IF NOT EXISTS {}
            (
                ID                  INTEGER PRIMARY KEY AUTOINCREMENT,
                ID_EDIFICIO         TEXT NOT NULL REFERENCES EDIFICIO (CHIAVE),
                TIPO                TEXT NOT NULL CHECK (TIPO IN ('idrica', 'termica', 'elettrica')),
                COD_CONTATORE       TEXT NOT NULL,
                INDIRIZZO_CONTATORE TEXT
            ) STRICT;", 
            Self::table_name()
        )
    }
}
impl ToRetrieveAll for Utenza {}
impl ToInsert for Utenza {
    fn to_insert() -> String {
        format!(
            "INSERT INTO {} (ID_EDIFICIO, TIPO, COD_CONTATORE, INDIRIZZO_CONTATORE) VALUES (?, ?, ?, ?) RETURNING *;",
            Self::table_name()
        )
    }

    fn to_insert_params(&self) -> Vec<&dyn SqlParams> {
        vec![
            &self.id_edificio,
            &self.tipo_cached,
            &self.cod_contatore,
            &self.indirizzo_contatore,
        ]
    }
}
impl ToUpdate for Utenza {
    fn to_update() -> String {
        panic!("Don't call this method. Use to_build_update instead")
    }

    fn to_build_update(
        &self,
    ) -> Result<Option<(String, Vec<Box<&dyn SqlParams>>)>, QueryBuilderError> {
        let mut set_clauses = Vec::new();
        let mut params: Vec<Box<&dyn SqlParams>> = Vec::new();

        set_clauses.push("COD_CONTATORE = ?");
        params.push(Box::new(&self.cod_contatore));

        // Aggiungere clausole dinamiche
        if let Some(ref indirizzo_contatore) = self.indirizzo_contatore {
            set_clauses.push("INDIRIZZO_CONTATORE = ?");
            params.push(Box::new(indirizzo_contatore));
        }

        // Se non ci sono campi da aggiornare, ritorniamo None
        if set_clauses.is_empty() {
            return Ok(None);
        }

        // Costruzione condizione WHERE
        let mut query = format!(
            "UPDATE {} SET {}",
            Self::table_name(),
            set_clauses.join(", ")
        );
        query.push_str(" WHERE ID = ?");
        params.push(Box::new(&self.id));

        query = format!("{} RETURNING *", query);

        Ok(Some((query, params)))
    }

    fn to_update_params(&self) -> Vec<Box<&dyn SqlParams>> {
        panic!("Don't call this method. Use to_build_update instead")
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TipoUtenza {
    Idrica,
    Elettrica,
    Termica,
}

impl From<&str> for TipoUtenza {
    fn from(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "idrica" => Self::Idrica,
            "elettrica" => Self::Elettrica,
            "termica" => Self::Termica,
            _ => panic!("TipoUtenza non valido"),
        }
    }
}

impl From<String> for TipoUtenza {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl Display for TipoUtenza {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Idrica => "idrica",
            Self::Elettrica => "elettrica",
            Self::Termica => "termica",
        }
        .to_string();
        write!(f, "{}", str)
    }
}

impl Serialize for TipoUtenza {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for TipoUtenza {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self::from(s))
    }
}
