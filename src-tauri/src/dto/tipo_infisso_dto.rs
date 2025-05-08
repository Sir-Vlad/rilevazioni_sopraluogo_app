use crate::dto::DTO;
use serde::Serialize;

#[derive(Serialize)]
pub struct TipoInfissiDTO {
    pub(crate) nome: String,
}

impl DTO for TipoInfissiDTO {}
