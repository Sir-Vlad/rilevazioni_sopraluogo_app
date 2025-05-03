use serde::{Deserialize, Serialize};
use crate::dto::DTO;

#[derive(Debug, Deserialize, Serialize)]
pub struct VetroInfissoDTO {
    pub vetro: String,
    pub efficienza_energetica: u8,
}

impl DTO for VetroInfissoDTO {}