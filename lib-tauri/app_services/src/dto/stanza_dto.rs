use app_macro::Builder;
use app_models::models::{NewStanza, Stanza, UpdateStanza};
use app_utils::app_interface::dto_interface::DTO;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Builder, PartialEq)]
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

impl<T> From<T> for StanzaDTO
where
    T: AsRef<Stanza>,
{
    fn from(value: T) -> Self {
        let stanza = value.as_ref();

        Self {
            id: stanza.id as u64,
            edificio_id: stanza.edificio_id.clone(),
            piano: stanza.piano.clone(),
            id_spazio: stanza.id_spazio.clone(),
            cod_stanza: stanza.cod_stanza.clone(),
            destinazione_uso: stanza.destinazione_uso.clone(),
            altezza: stanza.altezza.map(|t| t as u16),
            spessore_muro: stanza.spessore_muro.map(|t| t as u8),
            riscaldamento: stanza.riscaldamento.clone(),
            raffrescamento: stanza.raffrescamento.clone(),
            illuminazione: stanza.illuminazione.clone(),
            infissi: None,
        }
    }
}

impl From<StanzaDTO> for Stanza {
    fn from(value: StanzaDTO) -> Self {
        Self {
            id: value.id as i32,
            edificio_id: value.edificio_id.clone(),
            piano: value.piano.clone(),
            id_spazio: value.id_spazio.clone(),
            cod_stanza: value.cod_stanza.clone(),
            destinazione_uso: value.destinazione_uso.clone(),
            altezza: value.altezza.map(|t| t as i16),
            spessore_muro: value.spessore_muro.map(|t| t as i16),
            riscaldamento: value.riscaldamento.clone(),
            raffrescamento: value.raffrescamento.clone(),
            illuminazione: value.illuminazione.clone(),
        }
    }
}

impl From<StanzaDTO> for NewStanza<'_> {
    fn from(value: StanzaDTO) -> Self {
        Self {
            edificio_id: value.edificio_id.into(),
            piano: value.piano.into(),
            id_spazio: value.id_spazio.into(),
            cod_stanza: value.cod_stanza.into(),
            destinazione_uso: value.destinazione_uso.into(),
        }
    }
}

impl From<StanzaDTO> for UpdateStanza<'_> {
    fn from(value: StanzaDTO) -> Self {
        Self {
            altezza: value.altezza.map(|v| v as i16),
            spessore_muro: value.spessore_muro.map(|v| v as i16),
            riscaldamento: value.riscaldamento.map(|x| x.into()),
            raffrescamento: value.raffrescamento.map(|x| x.into()),
            illuminazione: value.illuminazione.map(|x| x.into()),
        }
    }
}
