use crate::dao::Infisso;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InfissoDto {
    pub id: String,
    pub tipo: String,
    pub altezza: i32,
    pub larghezza: i32,
    pub materiale: String,
    pub vetro: String,
}

impl From<&Infisso> for InfissoDto {
    fn from(infisso: &Infisso) -> Self {
        InfissoDto {
            id: infisso.id.clone(),
            tipo: infisso.tipo.clone(),
            altezza: infisso.altezza,
            larghezza: infisso.larghezza,
            materiale: infisso.materiale.clone(),
            vetro: infisso.vetro.clone(),
        }
    }
}
