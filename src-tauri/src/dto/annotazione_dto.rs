use crate::dao::entity::{AnnotazioneEdificio, AnnotazioneInfisso, AnnotazioneStanza};
use crate::dto::DTO;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AnnotazioneDTO {
    id: u64,
    ref_table: String,
    id_ref_table: String,
    content: String,
}

impl DTO for AnnotazioneDTO {}

impl From<AnnotazioneEdificioDTO> for AnnotazioneDTO {
    fn from(dto: AnnotazioneEdificioDTO) -> Self {
        Self {
            id: dto.id,
            ref_table: "edificio".to_string(),
            id_ref_table: dto.id_edificio,
            content: dto.content,
        }
    }
}

impl From<AnnotazioneStanzaDTO> for AnnotazioneDTO {
    fn from(dto: AnnotazioneStanzaDTO) -> Self {
        Self {
            id: dto.id,
            ref_table: "stanza".to_string(),
            id_ref_table: dto.id_stanza.to_string(),
            content: dto.content,
        }
    }
}

impl From<AnnotazioneInfissoDTO> for AnnotazioneDTO {
    fn from(dto: AnnotazioneInfissoDTO) -> Self {
        Self {
            id: dto.id,
            ref_table: "infisso".to_string(),
            id_ref_table: dto.id_infisso,
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

#[derive(Clone)]
pub struct AnnotazioneInfissoDTO {
    pub(crate) id: u64,
    pub(crate) id_infisso: String,
    pub(crate) content: String,
}

impl DTO for AnnotazioneInfissoDTO {}

impl From<AnnotazioneInfisso> for AnnotazioneInfissoDTO {
    fn from(dto: AnnotazioneInfisso) -> Self {
        Self {
            id: dto.id,
            id_infisso: dto.id_infisso,
            content: dto.content,
        }
    }
}
