use crate::dto::FotovoltaicoDTO;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Fotovoltaico {
    pub(crate) id: u64,
    pub(crate) id_edificio: String,
    pub(crate) potenza: u16,
    pub(crate) proprietario: String,
}

impl Fotovoltaico {
    pub fn new(id_edificio: &str, potenza: u16, proprietario: &str) -> Self {
        Self {
            id: 0,
            id_edificio: id_edificio.to_string(),
            potenza,
            proprietario: proprietario.to_string(),
        }
    }
}

impl From<FotovoltaicoDTO> for Fotovoltaico {
    fn from(value: FotovoltaicoDTO) -> Self {
        Self{
            id: value.id.unwrap_or(0),
            id_edificio: value.id_edificio,
            potenza: value.potenza,
            proprietario: value.proprietario,
        }
    }
}