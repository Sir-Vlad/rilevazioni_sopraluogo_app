use app_utils::app_interface::dto_interface::DTO;
use app_models::models::{NewStanza, Stanza, UpdateStanza};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StanzaDTO {
    pub id: u64,
    pub edificio_id: String,
    pub piano: String,
    pub id_spazio: String,
    pub cod_stanza: String,
    pub destinazione_uso: String,
    pub altezza: Option<u16>,
    pub spessore_muro: Option<u8>,
    pub riscaldamento: Option<String>,
    pub raffrescamento: Option<String>,
    pub illuminazione: Option<String>,
    pub infissi: Option<Vec<String>>,
}

impl DTO for StanzaDTO {}

impl From<&Stanza> for StanzaDTO {
    fn from(value: &Stanza) -> Self {
        Self {
            id: value.id as u64,
            edificio_id: value.edificio_id.clone(),
            piano: value.piano.clone(),
            id_spazio: value.id_spazio.clone(),
            cod_stanza: value.cod_stanza.clone(),
            destinazione_uso: value.destinazione_uso.clone(),
            altezza: value.altezza.map(|t| t as u16),
            spessore_muro: value.spessore_muro.map(|t| t as u8),
            riscaldamento: value.riscaldamento.clone(),
            raffrescamento: value.raffrescamento.clone(),
            illuminazione: value.illuminazione.clone(),
            infissi: None,
        }
    }
}

impl From<StanzaDTO> for NewStanza {
    fn from(value: StanzaDTO) -> Self {
        Self {
            edificio_id: value.edificio_id,
            piano: value.piano,
            id_spazio: value.id_spazio,
            cod_stanza: value.cod_stanza,
            destinazione_uso: value.destinazione_uso,
        }
    }
}

impl From<StanzaDTO> for UpdateStanza {
    fn from(value: StanzaDTO) -> Self {
        Self {
            altezza: value.altezza.map(|v| v as i16),
            spessore_muro: value.spessore_muro.map(|v| v as i16),
            riscaldamento: value.riscaldamento,
            raffrescamento: value.raffrescamento,
            illuminazione: value.illuminazione,
        }
    }
}
