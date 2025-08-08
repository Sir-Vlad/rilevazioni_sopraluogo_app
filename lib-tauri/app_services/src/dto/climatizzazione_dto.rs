use app_interface::dto_interface::DTO;
use app_models::models::Climatizzazione;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
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