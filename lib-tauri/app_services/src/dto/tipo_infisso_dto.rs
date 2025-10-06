use app_utils::app_interface::dto_interface::DTO;
use serde::Serialize;

#[derive(Serialize)]
pub struct TipoInfissiDTO {
    pub(crate) nome: String,
}

impl DTO for TipoInfissiDTO {}
