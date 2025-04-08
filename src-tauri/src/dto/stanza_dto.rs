use serde::{Deserialize, Serialize};
use crate::dao::Stanza;

#[derive(Debug, Serialize, Deserialize)]
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

impl From<&Stanza> for StanzaDto {
    fn from(value: &Stanza) -> Self {
        StanzaDto{
            id: value.id,
            chiave: value.chiave.clone(),
            piano: value.piano.clone(),
            id_spazio: value.id_spazio.clone(),
            stanza: value.stanza.clone(),
            destinazione_uso: value.destinazione_uso.clone(),
            altezza: value.altezza,
            spessore_muro: value.spessore_muro,
            riscaldamento: value.riscaldamento.clone(),
            raffrescamento: value.raffrescamento.clone(),
            illuminazione: value.illuminazione.clone(),
            infissi: None,
        }
    }
}
