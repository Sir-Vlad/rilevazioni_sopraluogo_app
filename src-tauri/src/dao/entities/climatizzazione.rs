use crate::dto::TipoDTO;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Climatizzazione {
    pub(crate) _id: Option<u64>,
    pub(crate) climatizzazione: String,
    pub(crate) efficienza_energetica: u8,
}

impl From<TipoDTO> for Climatizzazione {
    fn from(value: TipoDTO) -> Self {
        Self {
            _id: None,
            climatizzazione: value.name.clone(),
            efficienza_energetica: value.efficienza_energetica,
        }
    }
}
