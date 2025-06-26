use crate::app_traits::{DtoTrait, FromEntity};
use crate::entities::Infisso;
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

impl DtoTrait for InfissoDTO {
    type EntityLinked = Infisso;
}

impl FromEntity for InfissoDTO {
    fn from_entity(entity: <Self as DtoTrait>::EntityLinked) -> Self {
        Self {
            id: entity.id.clone(),
            id_edificio: entity.edificio_id.clone(),
            tipo: entity.tipo.clone(),
            altezza: entity.altezza,
            larghezza: entity.larghezza,
            materiale: entity.materiale.clone(),
            vetro: entity.vetro.clone(),
        }
    }
}
