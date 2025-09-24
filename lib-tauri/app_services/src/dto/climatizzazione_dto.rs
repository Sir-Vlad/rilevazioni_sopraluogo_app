use app_macro::Builder;
use app_models::models::Climatizzazione;
use app_utils::app_interface::dto_interface::DTO;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Builder)]
pub struct ClimatizzazioneDTO {
    pub climatizzazione: String,
    pub efficienza_energetica: u8,
}

impl DTO for ClimatizzazioneDTO {}

impl From<Climatizzazione> for ClimatizzazioneDTO {
    fn from(value: Climatizzazione) -> Self {
        Self {
            climatizzazione: value.nome,
            efficienza_energetica: value.eff_energetica as u8,
        }
    }
}
