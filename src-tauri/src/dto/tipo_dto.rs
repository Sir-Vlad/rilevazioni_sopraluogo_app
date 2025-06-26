use crate::app_traits::DtoTrait;
use crate::entities::{Climatizzazione, Illuminazione};
use crate::service::TypeDTO;
use serde::{Deserialize, Serialize};

pub enum TipoEntity {
    Climatizzazione(Climatizzazione),
    Illuminazione(Illuminazione),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TipoDTO {
    pub(crate) tipo: TypeDTO,
    pub(crate) name: String,
    pub(crate) efficienza_energetica: u8,
}

impl DtoTrait for TipoDTO {
    type EntityLinked = TipoEntity;
}

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

impl Into<Climatizzazione> for TipoDTO {
    fn into(self) -> Climatizzazione {
        Climatizzazione {
            _id: None,
            efficienza_energetica: self.efficienza_energetica,
            climatizzazione: self.name,
        }
    }
}
