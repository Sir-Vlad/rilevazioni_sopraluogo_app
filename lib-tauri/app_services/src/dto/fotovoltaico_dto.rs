use app_utils::app_interface::dto_interface::DTO;
use app_models::models::{Fotovoltaico, NewFotovoltaico};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct FotovoltaicoDTO {
    pub(crate) id: Option<u64>,
    pub(crate) id_edificio: String,
    pub(crate) potenza: f32,
    pub(crate) proprietario: String,
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

impl From<FotovoltaicoDTO> for NewFotovoltaico {
    fn from(value: FotovoltaicoDTO) -> Self {
        Self {
            edificio_id: value.id_edificio,
            potenza: value.potenza,
            proprietario: value.proprietario,
        }
    }
}