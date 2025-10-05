use app_models::models::{NewUtenza, TipoUtenza, Utenza};
use app_utils::app_interface::dto_interface::DTO;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct UtenzaDTO {
    pub id: u64,
    pub edificio_id: String,
    pub tipo: TipoUtenza,
    pub cod_contatore: String,
    pub indirizzo_contatore: Option<String>,
}

impl DTO for UtenzaDTO {}

impl From<&Utenza> for UtenzaDTO {
    fn from(value: &Utenza) -> Self {
        Self {
            id: value.id as u64,
            edificio_id: value.edificio_id.clone(),
            tipo: value.tipo.clone(),
            cod_contatore: value.cod_contatore.clone(),
            indirizzo_contatore: value.indirizzo_contatore.clone(),
        }
    }
}

impl From<UtenzaDTO> for NewUtenza {
    fn from(value: UtenzaDTO) -> Self {
        Self {
            edificio_id: value.edificio_id,
            tipo: value.tipo,
            cod_contatore: value.cod_contatore,
            indirizzo_contatore: value.indirizzo_contatore,
        }
    }
}
