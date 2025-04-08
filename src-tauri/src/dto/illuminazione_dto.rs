use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct IlluminazioneDto {
    pub lampadina: String,
    pub efficienza_energetica: i8,
}
