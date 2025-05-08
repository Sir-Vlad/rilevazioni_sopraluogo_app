use crate::dto::UtenzaDTO;
use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Utenza {
    pub(crate) id: u64,
    pub(crate) id_edificio: String,
    pub(crate) tipo: TipoUtenza,
    pub(crate) cod_contatore: String,
    pub(crate) indirizzo_contatore: Option<String>,
}

impl Utenza {
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
        }
    }
}

impl From<UtenzaDTO> for Utenza {
    fn from(value: UtenzaDTO) -> Self {
        Self {
            id: value.id.unwrap_or(0),
            id_edificio: value.id_edificio,
            tipo: value.tipo,
            cod_contatore: value.cod_contatore,
            indirizzo_contatore: value.indirizzo_contatore,
        }
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
            "idrica" | "acqua" => Self::Idrica,
            "elettrica" | "elettricità" => Self::Elettrica,
            "termica" | "calore" => Self::Termica,
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
            Self::Idrica => "acqua",
            Self::Elettrica => "elettricità",
            Self::Termica => "calore",
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