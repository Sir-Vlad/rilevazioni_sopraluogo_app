use serde::{Deserialize, Serialize};
use app_interface::dto_interface::DTO;
use app_models::models::{TipoUtenza, Utenza};

#[derive(Serialize, Deserialize, Clone)]
pub struct UtenzaDTO {
    pub(crate) id: u64,
    pub(crate) edificio_id: String,
    pub(crate) tipo: TipoUtenza,
    pub(crate) cod_contatore: String,
    pub(crate) indirizzo_contatore: Option<String>,
}

impl DTO for UtenzaDTO {}

impl From<&Utenza> for UtenzaDTO {
    fn from(value: &Utenza) -> Self {
        Self{
            id: value.id as u64,
            edificio_id: value.edificio_id.clone(),
            tipo: value.tipo.clone(),
            cod_contatore: value.cod_contatore.clone(),
            indirizzo_contatore: value.indirizzo_contatore.clone(),
        }
    }
}
