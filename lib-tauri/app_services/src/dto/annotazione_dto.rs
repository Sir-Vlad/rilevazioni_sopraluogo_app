use std::fmt::{Display, Formatter};

use app_models::models::{
    AnnotazioneEdificio, AnnotazioneInfisso, AnnotazioneStanza, NewAnnotazioneEdificio,
    NewAnnotazioneInfisso, NewAnnotazioneStanza,
};
use app_utils::app_interface::dto_interface::DTO;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum TableWithPrimaryKey {
    Edificio(String),
    Stanza(u64),
    Infisso((String, String)),
}

impl Display for TableWithPrimaryKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TableWithPrimaryKey::Edificio(_) => f.write_str("Edificio"),
            TableWithPrimaryKey::Stanza(_) => f.write_str("Stanza"),
            TableWithPrimaryKey::Infisso(_) => f.write_str("Infisso"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AnnotazioneDTO {
    id: u64,
    /// tabella specifica della annotazione
    pub ref_table: TableWithPrimaryKey,
    /// contenuto dell'annotazione
    content: String,
}

impl DTO for AnnotazioneDTO {}

impl From<AnnotazioneEdificioDTO> for AnnotazioneDTO {
    fn from(dto: AnnotazioneEdificioDTO) -> Self {
        Self {
            id: dto.id,
            ref_table: TableWithPrimaryKey::Edificio(dto.edificio_id),
            content: dto.content,
        }
    }
}

impl From<AnnotazioneStanzaDTO> for AnnotazioneDTO {
    fn from(dto: AnnotazioneStanzaDTO) -> Self {
        Self {
            id: dto.id,
            ref_table: TableWithPrimaryKey::Stanza(dto.stanza_id),
            content: dto.content,
        }
    }
}

impl From<AnnotazioneInfissoDTO> for AnnotazioneDTO {
    fn from(dto: AnnotazioneInfissoDTO) -> Self {
        Self {
            id: dto.id,
            ref_table: TableWithPrimaryKey::Infisso((dto.infisso_id, dto.edificio_id)),
            content: dto.content,
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(test, derive(Deserialize))]
pub struct AnnotazioneEdificioDTO {
    pub(crate) id: u64,
    pub(crate) edificio_id: String,
    pub(crate) content: String,
}

impl DTO for AnnotazioneEdificioDTO {}

impl From<AnnotazioneEdificio> for AnnotazioneEdificioDTO {
    fn from(dto: AnnotazioneEdificio) -> Self {
        Self {
            id: dto.id as u64,
            edificio_id: dto.edificio_id,
            content: dto.content,
        }
    }
}

impl From<AnnotazioneEdificioDTO> for NewAnnotazioneEdificio<'_> {
    fn from(dto: AnnotazioneEdificioDTO) -> Self {
        Self {
            edificio_id: dto.edificio_id.into(),
            content: dto.content.into(),
        }
    }
}

impl From<AnnotazioneDTO> for AnnotazioneEdificioDTO {
    fn from(dto: AnnotazioneDTO) -> Self {
        if let TableWithPrimaryKey::Edificio(id_edificio) = dto.ref_table {
            Self {
                id: dto.id,
                edificio_id: id_edificio,
                content: dto.content,
            }
        } else {
            log::error!("Errore nella conversione AnnotazioneDTO -> AnnotazioneEdificioDTO");
            Self {
                id: dto.id,
                edificio_id: String::new(),
                content: dto.content,
            }
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(test, derive(Deserialize))]
pub struct AnnotazioneStanzaDTO {
    pub(crate) id: u64,
    pub(crate) stanza_id: u64,
    pub(crate) content: String,
}

impl DTO for AnnotazioneStanzaDTO {}

impl From<AnnotazioneStanza> for AnnotazioneStanzaDTO {
    fn from(dto: AnnotazioneStanza) -> Self {
        Self {
            id: dto.id as u64,
            stanza_id: dto.stanza_id as u64,
            content: dto.content,
        }
    }
}

impl From<AnnotazioneStanzaDTO> for NewAnnotazioneStanza<'_> {
    fn from(value: AnnotazioneStanzaDTO) -> Self {
        Self {
            stanza_id: value.stanza_id as i32,
            content: value.content.into(),
        }
    }
}

impl From<AnnotazioneDTO> for AnnotazioneStanzaDTO {
    fn from(dto: AnnotazioneDTO) -> Self {
        if let TableWithPrimaryKey::Stanza(id_stanza) = dto.ref_table {
            Self {
                id: dto.id,
                stanza_id: id_stanza,
                content: dto.content,
            }
        } else {
            log::error!("Errore nella conversione AnnotazioneDTO -> AnnotazioneStanzaDTO");
            Self {
                id: dto.id,
                stanza_id: 0,
                content: dto.content,
            }
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(test, derive(Deserialize))]
pub struct AnnotazioneInfissoDTO {
    pub(crate) id: u64,
    pub(crate) infisso_id: String,
    pub(crate) edificio_id: String,
    pub(crate) content: String,
}

impl DTO for AnnotazioneInfissoDTO {}

impl From<AnnotazioneInfisso> for AnnotazioneInfissoDTO {
    fn from(dto: AnnotazioneInfisso) -> Self {
        Self {
            id: dto.id as u64,
            infisso_id: dto.infisso_id,
            edificio_id: dto.edificio_id,
            content: dto.content,
        }
    }
}

impl From<AnnotazioneInfissoDTO> for NewAnnotazioneInfisso<'_> {
    fn from(value: AnnotazioneInfissoDTO) -> Self {
        Self {
            infisso_id: value.infisso_id.into(),
            edificio_id: value.edificio_id.into(),
            content: value.content.into(),
        }
    }
}

impl From<AnnotazioneDTO> for AnnotazioneInfissoDTO {
    fn from(dto: AnnotazioneDTO) -> Self {
        if let TableWithPrimaryKey::Infisso((id_infisso, edificio)) = dto.ref_table {
            Self {
                id: dto.id,
                infisso_id: id_infisso,
                edificio_id: edificio,
                content: dto.content,
            }
        } else {
            log::error!("Errore nella conversione AnnotazioneDTO -> AnnotazioneInfissoDTO");
            Self {
                id: dto.id,
                infisso_id: String::new(),
                edificio_id: String::new(),
                content: dto.content,
            }
        }
    }
}
