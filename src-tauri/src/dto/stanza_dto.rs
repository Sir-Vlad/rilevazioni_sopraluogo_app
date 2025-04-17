use crate::dao::Stanza;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StanzaDto {
    pub id: u64,
    pub chiave: String,
    pub piano: String,
    pub id_spazio: String,
    pub stanza: String,
    pub destinazione_uso: String,
    pub altezza: Option<u16>,
    pub spessore_muro: Option<u8>,
    pub riscaldamento: Option<String>,
    pub raffrescamento: Option<String>,
    pub illuminazione: Option<String>,
    pub infissi: Option<Vec<String>>,
}

impl StanzaDto {
    fn from_stanza_common(stanza: &Stanza) -> Self {
        StanzaDto {
            id: stanza.id.unwrap_or(0),
            chiave: stanza.chiave.clone(),
            piano: stanza.piano.clone(),
            id_spazio: stanza.id_spazio.clone(),
            stanza: stanza.stanza.clone(),
            destinazione_uso: stanza.destinazione_uso.clone(),
            altezza: stanza.altezza,
            spessore_muro: stanza.spessore_muro,
            riscaldamento: stanza.riscaldamento.clone(),
            raffrescamento: stanza.raffrescamento.clone(),
            illuminazione: stanza.illuminazione.clone(),
            infissi: None,
        }
    }

}

impl From<Stanza> for StanzaDto {
    fn from(value: Stanza) -> Self {
        StanzaDto::from_stanza_common(&value)
    }
}

impl From<&Stanza> for StanzaDto {
    fn from(value: &Stanza) -> Self {
        StanzaDto::from_stanza_common(value)   
    }
}
