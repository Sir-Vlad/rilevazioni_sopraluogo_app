use app_models::models::{Fotovoltaico, NewFotovoltaico};
use app_utils::app_interface::dto_interface::DTO;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct FotovoltaicoDTO {
    pub id: Option<u64>,
    pub id_edificio: String,
    pub potenza: f32,
    pub proprietario: String,
}

impl DTO for FotovoltaicoDTO {}

impl From<&Fotovoltaico> for FotovoltaicoDTO {
    fn from(value: &Fotovoltaico) -> Self {
        Self {
            id: Some(value.id as u64),
            id_edificio: value.edificio_id.clone(),
            potenza: value.potenza,
            proprietario: value.proprietario.clone(),
        }
    }
}

impl From<FotovoltaicoDTO> for NewFotovoltaico<'_> {
    fn from(value: FotovoltaicoDTO) -> Self {
        Self {
            edificio_id: value.id_edificio.into(),
            potenza: value.potenza,
            proprietario: value.proprietario.into(),
        }
    }
}
