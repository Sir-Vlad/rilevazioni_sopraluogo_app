use app_models::models::Illuminazione;
use app_utils::app_interface::dto_interface::DTO;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct IlluminazioneDTO {
    pub lampadina: String,
    pub efficienza_energetica: u8,
}

impl DTO for IlluminazioneDTO {}

impl From<Illuminazione> for IlluminazioneDTO {
    fn from(value: Illuminazione) -> Self {
        Self {
            lampadina: value.lampadina,
            efficienza_energetica: value.eff_energetica as u8,
        }
    }
}
