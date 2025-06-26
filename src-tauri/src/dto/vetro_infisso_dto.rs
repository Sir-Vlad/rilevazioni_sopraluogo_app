use crate::app_traits::{DtoTrait, FromEntity};
use crate::entities::VetroInfisso;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct VetroInfissoDTO {
    pub vetro: String,
    pub efficienza_energetica: u8,
}

impl DtoTrait for VetroInfissoDTO {
    type EntityLinked = VetroInfisso;
}

impl FromEntity for VetroInfissoDTO {
    fn from_entity(entity: <Self as DtoTrait>::EntityLinked) -> Self {
        Self {
            vetro: entity.vetro,
            efficienza_energetica: entity.efficienza_energetica,
        }
    }
}
