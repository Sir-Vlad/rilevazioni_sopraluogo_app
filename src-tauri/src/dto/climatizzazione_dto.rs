use crate::app_traits::{DtoTrait, FromEntity};
use crate::entities::Climatizzazione;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ClimatizzazioneDTO {
    pub climatizzazione: String,
    pub efficienza_energetica: u8,
}

impl DtoTrait for ClimatizzazioneDTO {
    type EntityLinked = Climatizzazione;
}

impl FromEntity for ClimatizzazioneDTO {
    fn from_entity(entity: <Self as DtoTrait>::EntityLinked) -> Self {
        Self {
            climatizzazione: entity.climatizzazione,
            efficienza_energetica: entity.efficienza_energetica,
        }
    }
}
