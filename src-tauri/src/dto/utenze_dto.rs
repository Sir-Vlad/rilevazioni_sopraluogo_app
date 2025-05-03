use crate::dao::entity::TipoUtenza;
use crate::dto::DTO;

pub struct UtenzaDTO {
    pub(crate) id: u64,
    pub(crate) id_edificio: String,
    pub(crate) tipo: TipoUtenza,
    pub(crate) cod_contatore: String,
    pub(crate) indirizzo_contatore: Option<String>,
}

impl DTO for UtenzaDTO {}
