use crate::dto::TipoDTO;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Illuminazione {
    pub(crate) _id: Option<u64>,
    pub(crate) lampadina: String,
    pub(crate) efficienza_energetica: u8,
}

impl From<TipoDTO> for Illuminazione {
    fn from(value: TipoDTO) -> Self {
        Self {
            _id: None,
            lampadina: value.name.clone(),
            efficienza_energetica: value.efficienza_energetica,
        }
    }
}
