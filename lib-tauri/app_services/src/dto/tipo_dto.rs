use crate::dao::entity::{Climatizzazione, Illuminazione};
use crate::dto::DTO;
use crate::service::TypeDTO;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TipoDTO {
    pub(crate) tipo: TypeDTO,
    pub(crate) name: String,
    pub(crate) efficienza_energetica: u8,
}

impl DTO for TipoDTO {}

impl TipoDTO {
    #[cfg(test)]
    pub fn new(tipo: String, name: String, efficienza_energetica: u8) -> Self {
        Self {
            tipo: TypeDTO::try_from(tipo).ok().unwrap(),
            name,
            efficienza_energetica,
        }
    }
}

impl From<Climatizzazione> for TipoDTO {
    fn from(value: Climatizzazione) -> Self {
        Self {
            tipo: TypeDTO::Climatizzazione,
            name: value.climatizzazione,
            efficienza_energetica: value.efficienza_energetica,
        }
    }
}

impl From<Illuminazione> for TipoDTO {
    fn from(value: Illuminazione) -> Self {
        Self {
            tipo: TypeDTO::Illuminazione,
            name: value.lampadina,
            efficienza_energetica: value.efficienza_energetica,
        }
    }
}
