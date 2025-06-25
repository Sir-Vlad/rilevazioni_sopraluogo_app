use crate::entities::{AnnotazioneEdificio, AnnotazioneInfisso, AnnotazioneStanza};
use crate::dto::DTO;
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
            id_ref_table: PrimaryKey::Edificio(dto.id_edificio),
            content: dto.content,
        }
    }
}

impl From<AnnotazioneStanzaDTO> for AnnotazioneDTO {
    fn from(dto: AnnotazioneStanzaDTO) -> Self {
        Self {
            id: dto.id,
            ref_table: "stanza".to_string(),
            id_ref_table: PrimaryKey::Stanza(dto.id_stanza),
            content: dto.content,
        }
    }
}

impl From<AnnotazioneInfissoDTO> for AnnotazioneDTO {
    fn from(dto: AnnotazioneInfissoDTO) -> Self {
        Self {
            id: dto.id,
            ref_table: "infisso".to_string(),
            id_ref_table: PrimaryKey::Infisso((dto.id_infisso, dto.edificio)),
            content: dto.content,
        }
    }
}

#[derive(Clone)]
pub struct AnnotazioneEdificioDTO {
    pub(crate) id: u64,
    pub(crate) id_edificio: String,
    pub(crate) content: String,
}

impl DTO for AnnotazioneEdificioDTO {}

impl From<AnnotazioneEdificio> for AnnotazioneEdificioDTO {
    fn from(dto: AnnotazioneEdificio) -> Self {
        Self {
            id: dto.id,
            id_edificio: dto.id_edificio,
            content: dto.content,
        }
    }
}

impl From<AnnotazioneDTO> for AnnotazioneEdificioDTO {
    fn from(dto: AnnotazioneDTO) -> Self {
        if let PrimaryKey::Edificio(id_edificio) = dto.id_ref_table {
            Self {
                id: dto.id,
                id_edificio,
                content: dto.content,
            }
        } else {
            log::error!("Errore nella conversione AnnotazioneDTO -> AnnotazioneEdificioDTO");
            Self {
                id: dto.id,
                id_edificio: String::new(),
                content: dto.content,
            }
        }
    }
}

#[derive(Clone)]
pub struct AnnotazioneStanzaDTO {
    pub(crate) id: u64,
    pub(crate) id_stanza: u64,
    pub(crate) content: String,
}

impl DTO for AnnotazioneStanzaDTO {}

impl From<AnnotazioneStanza> for AnnotazioneStanzaDTO {
    fn from(dto: AnnotazioneStanza) -> Self {
        Self {
            id: dto.id,
            id_stanza: dto.id_stanza,
            content: dto.content,
        }
    }
}

impl From<AnnotazioneDTO> for AnnotazioneStanzaDTO {
    fn from(dto: AnnotazioneDTO) -> Self {
        if let PrimaryKey::Stanza(id_stanza) = dto.id_ref_table {
            Self {
                id: dto.id,
                id_stanza,
                content: dto.content,
            }
        } else {
            log::error!("Errore nella conversione AnnotazioneDTO -> AnnotazioneStanzaDTO");
            Self {
                id: dto.id,
                id_stanza: 0,
                content: dto.content,
            }
        }
    }
}

#[derive(Clone)]
pub struct AnnotazioneInfissoDTO {
    pub(crate) id: u64,
    pub(crate) id_infisso: String,
    pub(crate) edificio: String,
    pub(crate) content: String,
}

impl DTO for AnnotazioneInfissoDTO {}

impl From<AnnotazioneInfisso> for AnnotazioneInfissoDTO {
    fn from(dto: AnnotazioneInfisso) -> Self {
        Self {
            id: dto.id,
            id_infisso: dto.id_infisso,
            edificio: dto.edificio,
            content: dto.content,
        }
    }
}

impl From<AnnotazioneDTO> for AnnotazioneInfissoDTO {
    fn from(dto: AnnotazioneDTO) -> Self {
        if let PrimaryKey::Infisso((id_infisso, edificio)) = dto.id_ref_table {
            Self {
                id: dto.id,
                id_infisso,
                edificio,
                content: dto.content,
            }
        } else {
            log::error!("Errore nella conversione AnnotazioneDTO -> AnnotazioneInfissoDTO");
            Self {
                id: dto.id,
                id_infisso: String::new(),
                edificio: String::new(),
                content: dto.content,
            }
        }
    }
}
