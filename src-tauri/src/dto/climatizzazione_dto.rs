use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ClimatizzazioneDto {
    pub climatizzazione: String,
    pub efficienza_energetica: i8,
}
