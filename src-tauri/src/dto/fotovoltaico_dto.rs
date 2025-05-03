use crate::dao::entity::Fotovoltaico;
use crate::dto::DTO;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct FotovoltaicoDTO {
    pub(crate) id: Option<u64>,
    pub(crate) id_edificio: String,
    pub(crate) potenza: u16,
    pub(crate) proprietario: String,
}

impl DTO for FotovoltaicoDTO {}

impl From<&Fotovoltaico> for FotovoltaicoDTO {
    fn from(value: &Fotovoltaico) -> Self {
        Self{
            id: Some(value.id),
            id_edificio: value.id_edificio.clone(),
            potenza: value.potenza,
            proprietario: value.proprietario.clone(),
        }
    }
}
