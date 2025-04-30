use std::fmt::Display;

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
