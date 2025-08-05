use serde::{Deserialize, Serialize};
use crate::dao::entity::{TipoUtenza, Utenza};
use crate::dto::DTO;

#[derive(Serialize, Deserialize, Clone)]
pub struct UtenzaDTO {
    pub(crate) id: Option<u64>,
    pub(crate) id_edificio: String,
    pub(crate) tipo: TipoUtenza,
    pub(crate) cod_contatore: String,
    pub(crate) indirizzo_contatore: Option<String>,
}

impl DTO for UtenzaDTO {}

impl From<&Utenza> for UtenzaDTO {
    fn from(value: &Utenza) -> Self {
        Self{
            id: Some(value.id),
            id_edificio: value.id_edificio.clone(),
            tipo: value.tipo.clone(),
            cod_contatore: value.cod_contatore.clone(),
            indirizzo_contatore: value.indirizzo_contatore.clone(),
        }
    }
}
