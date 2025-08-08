use app_interface::dto_interface::DTO;
use app_models::models::VetroInfisso;
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
            efficienza_energetica: value.eff_energetica as u8,
        }
    }
}
