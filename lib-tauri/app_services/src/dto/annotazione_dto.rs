use app_utils::app_interface::dto_interface::DTO;
use app_models::models::{AnnotazioneEdificio, AnnotazioneInfisso, AnnotazioneStanza, NewAnnotazioneEdificio, NewAnnotazioneInfisso, NewAnnotazioneStanza};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum PrimaryKey {
    Edificio(String),
    Stanza(u64),
    Infisso((String, String)),
}

#[derive(Serialize, Deserialize)]
pub struct AnnotazioneDTO {
    id: u64,
    /// tabella specifica della annotazione
    pub(crate) ref_table: String,
    /// riferimento alla colonna della tabella
    id_ref_table: PrimaryKey,
    /// contenuto dell'annotazione
    content: String,
}

impl DTO for AnnotazioneDTO {}

impl From<AnnotazioneEdificioDTO> for AnnotazioneDTO {
    fn from(dto: AnnotazioneEdificioDTO) -> Self {
        Self {
            id: dto.id,
            ref_table: "edificio".to_string(),
            id_ref_table: PrimaryKey::Edificio(dto.edificio_id),
            content: dto.content,
        }
    }
}

impl From<AnnotazioneStanzaDTO> for AnnotazioneDTO {
    fn from(dto: AnnotazioneStanzaDTO) -> Self {
        Self {
            id: dto.id,
            ref_table: "stanza".to_string(),
            id_ref_table: PrimaryKey::Stanza(dto.stanza_id),
            content: dto.content,
        }
    }
}

impl From<AnnotazioneInfissoDTO> for AnnotazioneDTO {
    fn from(dto: AnnotazioneInfissoDTO) -> Self {
        Self {
            id: dto.id,
            ref_table: "infisso".to_string(),
            id_ref_table: PrimaryKey::Infisso((dto.infisso_id, dto.edificio_id)),
            content: dto.content,
        }
    }
}

#[derive(Clone)]
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

impl From<AnnotazioneEdificioDTO> for NewAnnotazioneEdificio {
    fn from(dto: AnnotazioneEdificioDTO) -> Self {
        Self {
            edificio_id: dto.edificio_id,
            content: dto.content,
        }
    }
}


impl From<AnnotazioneDTO> for AnnotazioneEdificioDTO {
    fn from(dto: AnnotazioneDTO) -> Self {
        if let PrimaryKey::Edificio(id_edificio) = dto.id_ref_table {
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

#[derive(Clone)]
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

impl From<AnnotazioneStanzaDTO> for NewAnnotazioneStanza {
    fn from(value: AnnotazioneStanzaDTO) -> Self {
        Self {
            stanza_id: value.stanza_id as i32,
            content: value.content,
        }
    }
}

impl From<AnnotazioneDTO> for AnnotazioneStanzaDTO {
    fn from(dto: AnnotazioneDTO) -> Self {
        if let PrimaryKey::Stanza(id_stanza) = dto.id_ref_table {
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

#[derive(Clone)]
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

impl From<AnnotazioneInfissoDTO> for NewAnnotazioneInfisso {
    fn from(value: AnnotazioneInfissoDTO) -> Self {
        Self {
            infisso_id: value.infisso_id,
            edificio_id: value.edificio_id,
            content: value.content,
        }
    }
}

impl From<AnnotazioneDTO> for AnnotazioneInfissoDTO {
    fn from(dto: AnnotazioneDTO) -> Self {
        if let PrimaryKey::Infisso((id_infisso, edificio)) = dto.id_ref_table {
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
