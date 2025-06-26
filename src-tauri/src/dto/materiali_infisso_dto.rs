use crate::app_traits::{DtoTrait, FromEntity};
use crate::entities::MaterialeInfisso;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MaterialeInfissoDTO {
    pub materiale: String,
    pub efficienza_energetica: u8,
}

impl DtoTrait for MaterialeInfissoDTO {
    type EntityLinked = MaterialeInfisso;
}

impl FromEntity for MaterialeInfissoDTO {
    fn from_entity(entity: <Self as DtoTrait>::EntityLinked) -> Self {
        Self {
            materiale: entity.materiale,
            efficienza_energetica: entity.efficienza_energetica,
        }
    }
}
