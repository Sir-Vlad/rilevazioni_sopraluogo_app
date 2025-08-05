use crate::dao::entity::VetroInfisso;
use crate::dto::DTO;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct VetroInfissoDTO {
    pub vetro: String,
    pub efficienza_energetica: u8,
}

impl DTO for VetroInfissoDTO {}

impl From<VetroInfisso> for VetroInfissoDTO {
    fn from(value: VetroInfisso) -> Self {
        Self {
            vetro: value.vetro,
            efficienza_energetica: value.efficienza_energetica,
        }
    }
}
