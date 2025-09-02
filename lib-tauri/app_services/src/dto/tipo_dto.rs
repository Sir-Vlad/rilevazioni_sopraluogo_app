use crate::service::TypeDTO;
use app_models::models::{Climatizzazione, Illuminazione};
use app_utils::app_interface::dto_interface::DTO;
use serde::{Deserialize, Serialize};
use crate::dao::IlluminazioneDAO;

#[derive(Debug, Serialize, Deserialize)]
pub struct TipoDTO {
    pub(crate) tipo: TypeDTO,
    pub(crate) name: String,
    pub(crate) eff_energetica: u8,
}

impl DTO for TipoDTO {}

impl TipoDTO {
    #[cfg(test)]
    pub fn new(tipo: String, name: String, efficienza_energetica: u8) -> Self {
        Self {
            tipo: TypeDTO::try_from(tipo).ok().unwrap(),
            name,
            eff_energetica: efficienza_energetica,
        }
    }
}

impl From<Climatizzazione> for TipoDTO {
    fn from(value: Climatizzazione) -> Self {
        Self {
            tipo: TypeDTO::Climatizzazione,
            name: value.nome,
            eff_energetica: value.eff_energetica as u8,
        }
    }
}

impl From<TipoDTO> for Climatizzazione {
    fn from(value: TipoDTO) -> Self {
        Self {
            nome: value.name,
            eff_energetica: value.eff_energetica as i16,
        }
    }
}


impl From<Illuminazione> for TipoDTO {
    fn from(value: Illuminazione) -> Self {
        Self {
            tipo: TypeDTO::Illuminazione,
            name: value.lampadina,
            eff_energetica: value.eff_energetica as u8,
        }
    }
}

impl From<TipoDTO> for Illuminazione {
    fn from(value: TipoDTO) -> Self {
        Self {
            lampadina: value.name,
            eff_energetica: value.eff_energetica as i16,
        }
    }
}