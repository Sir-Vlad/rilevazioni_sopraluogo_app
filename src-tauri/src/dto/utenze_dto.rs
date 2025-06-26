use crate::app_traits::{DtoTrait, FromEntity};
use crate::entities::{TipoUtenza, Utenza};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct UtenzaDTO {
    pub(crate) id: Option<u64>,
    pub(crate) id_edificio: String,
    pub(crate) tipo: TipoUtenza,
    pub(crate) cod_contatore: String,
    pub(crate) indirizzo_contatore: Option<String>,
}

impl DtoTrait for UtenzaDTO {
    type EntityLinked = Utenza;
}

impl FromEntity for UtenzaDTO {
    fn from_entity(entity: <Self as DtoTrait>::EntityLinked) -> Self {
        Self {
            id: Some(entity.id),
            id_edificio: entity.id_edificio.clone(),
            tipo: entity.tipo.clone(),
            cod_contatore: entity.cod_contatore.clone(),
            indirizzo_contatore: entity.indirizzo_contatore.clone(),
        }
    }
}
