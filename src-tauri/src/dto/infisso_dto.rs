use crate::dao::entity::Infisso;
use crate::dto::DTO;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InfissoDTO {
    pub id: String,
    pub tipo: String,
    pub altezza: u16,
    pub larghezza: u16,
    pub materiale: String,
    pub vetro: String,
}

impl DTO for InfissoDTO {}

impl From<&Infisso> for InfissoDTO {
    fn from(infisso: &Infisso) -> Self {
        InfissoDTO {
            id: infisso.id.clone(),
            tipo: infisso.tipo.clone(),
            altezza: infisso.altezza,
            larghezza: infisso.larghezza,
            materiale: infisso.materiale.clone(),
            vetro: infisso.vetro.clone(),
        }
    }
}
