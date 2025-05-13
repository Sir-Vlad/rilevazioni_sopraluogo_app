use crate::dto::{VetroInfissoDTO, DTO};
use serde::{Deserialize, Serialize};
use crate::dao::entity::{Climatizzazione, VetroInfisso};

#[derive(Debug, Deserialize, Serialize)]
pub struct ClimatizzazioneDTO {
    pub climatizzazione: String,
    pub efficienza_energetica: u8,
}

impl DTO for ClimatizzazioneDTO {}

impl From<Climatizzazione> for ClimatizzazioneDTO {
    fn from(value: Climatizzazione) -> Self {
        Self {
            climatizzazione: value.climatizzazione,
            efficienza_energetica: value.efficienza_energetica,
        }
    }
}