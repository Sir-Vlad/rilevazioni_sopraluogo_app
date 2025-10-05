use app_models::models::{Infisso, NewInfisso, UpdateInfisso};
use app_utils::app_interface::dto_interface::DTO;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(test, derive(PartialEq))]
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
            id: infisso.id.trim().to_string(),
            id_edificio: infisso.edificio_id.clone(),
            tipo: infisso.tipo.clone(),
            altezza: infisso.altezza as u16,
            larghezza: infisso.larghezza as u16,
            materiale: infisso.materiale.clone(),
            vetro: infisso.vetro.clone(),
        }
    }
}

impl From<InfissoDTO> for NewInfisso<'_> {
    fn from(value: InfissoDTO) -> Self {
        Self {
            id: value.id.into(),
            edificio_id: value.id_edificio.into(),
            tipo: value.tipo.into(),
            altezza: value.altezza as i16,
            larghezza: value.larghezza as i16,
            materiale: value.materiale.into(),
            vetro: value.vetro.into(),
        }
    }
}

impl From<InfissoDTO> for UpdateInfisso<'_> {
    fn from(value: InfissoDTO) -> Self {
        Self {
            altezza: Some(value.altezza as i16),
            larghezza: Some(value.larghezza as i16),
            materiale: Some(value.materiale.into()),
            vetro: Some(value.vetro.into()),
        }
    }
}
