use app_utils::app_interface::dto_interface::DTO;
use app_models::models::MaterialeInfisso;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MaterialeInfissoDTO {
    pub materiale: String,
    pub efficienza_energetica: u8,
}

impl DTO for MaterialeInfissoDTO {}

impl From<MaterialeInfisso> for MaterialeInfissoDTO {
    fn from(materiali_infisso: MaterialeInfisso) -> Self {
        Self {
            materiale: materiali_infisso.materiale,
            efficienza_energetica: materiali_infisso.eff_energetica as u8,
        }
    }
}