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
