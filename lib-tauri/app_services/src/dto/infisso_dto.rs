use app_utils::app_interface::dto_interface::DTO;
use app_models::models::{Infisso, NewInfisso, UpdateInfisso};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InfissoDTO {
    pub id: String,
    pub id_edificio: String,
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
            id_edificio: infisso.edificio_id.clone(),
            tipo: infisso.tipo.clone(),
            altezza: infisso.altezza as u16,
            larghezza: infisso.larghezza as u16,
            materiale: infisso.materiale.clone(),
            vetro: infisso.vetro.clone(),
        }
    }
}

impl From<InfissoDTO> for NewInfisso {
    fn from(value: InfissoDTO) -> Self {
        Self {
            id: value.id,
            edificio_id: value.id_edificio,
            tipo: value.tipo,
            altezza: value.altezza as i16,
            larghezza: value.larghezza as i16,
            materiale: value.materiale,
            vetro: value.vetro,
        }
    }
}

impl From<InfissoDTO> for UpdateInfisso {
    fn from(value: InfissoDTO) -> Self {
        Self {
            altezza: Some(value.altezza as i16),
            larghezza: Some(value.larghezza as i16),
            materiale: Some(value.materiale),
            vetro: Some(value.vetro),
        }
    }
}
