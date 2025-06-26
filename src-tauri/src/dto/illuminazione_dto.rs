use crate::app_traits::{DtoTrait, FromEntity};
use crate::entities::Illuminazione;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct IlluminazioneDTO {
    pub lampadina: String,
    pub efficienza_energetica: u8,
}

impl DtoTrait for IlluminazioneDTO {
    type EntityLinked = Illuminazione;
}

impl FromEntity for IlluminazioneDTO {
    fn from_entity(entity: <Self as DtoTrait>::EntityLinked) -> Self {
        Self {
            lampadina: entity.lampadina,
            efficienza_energetica: entity.efficienza_energetica,
        }
    }
}
