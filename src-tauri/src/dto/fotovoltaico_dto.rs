use crate::app_traits::{DtoTrait, FromEntity};
use crate::entities::Fotovoltaico;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct FotovoltaicoDTO {
    pub(crate) id: Option<u64>,
    pub(crate) id_edificio: String,
    pub(crate) potenza: f32,
    pub(crate) proprietario: String,
}

impl DtoTrait for FotovoltaicoDTO {
    type EntityLinked = Fotovoltaico;
}

impl FromEntity for FotovoltaicoDTO {
    fn from_entity(entity: <Self as DtoTrait>::EntityLinked) -> Self {
        Self {
            id: Some(entity.id),
            id_edificio: entity.id_edificio.clone(),
            potenza: entity.potenza,
            proprietario: entity.proprietario.clone(),
        }
    }
}
