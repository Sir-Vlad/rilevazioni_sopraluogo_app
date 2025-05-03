use crate::dto::DTO;

pub struct FotovoltaicoDTO {
    pub(crate) id: u64,
    pub(crate) id_edificio: String,
    pub(crate) potenza: u16,
    pub(crate) proprietario: String,
}

impl DTO for FotovoltaicoDTO {}
