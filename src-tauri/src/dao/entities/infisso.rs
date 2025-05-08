use crate::dto::InfissoDTO;
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Infisso {
    pub(crate) id: String,
    pub(crate) tipo: String,
    pub(crate) altezza: u16,
    pub(crate) larghezza: u16,
    pub(crate) materiale: String,
    pub(crate) vetro: String,
}

impl Infisso {
    pub fn new(id: &str, tipo: &str, altezza: u16, larghezza: u16, materiale: &str, vetro: &str) -> Self {
        Self {
            id: id.to_string(),
            tipo: tipo.to_string(),
            altezza,
            larghezza,
            materiale: materiale.to_string(),
            vetro: vetro.to_string(),
        }
    }
}


impl From<InfissoDTO> for Infisso {
    fn from(infisso: InfissoDTO) -> Self {
        Infisso {
            id: infisso.id.clone(),
            tipo: infisso.tipo.clone(),
            altezza: infisso.altezza,
            larghezza: infisso.larghezza,
            materiale: infisso.materiale.clone(),
            vetro: infisso.vetro.clone(),
        }
    }
}
