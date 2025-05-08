use crate::dto::DTO;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct IlluminazioneDTO {
    pub lampadina: String,
    pub efficienza_energetica: u8,
}

impl DTO for IlluminazioneDTO {}
