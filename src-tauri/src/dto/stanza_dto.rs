use crate::app_traits::{DtoTrait, FromEntity};
use crate::entities::Stanza;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StanzaDTO {
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

impl DtoTrait for StanzaDTO {
    type EntityLinked = Stanza;
}

impl FromEntity for StanzaDTO {
    fn from_entity(entity: <Self as DtoTrait>::EntityLinked) -> Self {
        Self {
            id: entity.id.unwrap_or(0),
            chiave: entity.chiave.clone(),
            piano: entity.piano.clone(),
            id_spazio: entity.id_spazio.clone(),
            stanza: entity.cod_stanza.clone(),
            destinazione_uso: entity.destinazione_uso.clone(),
            altezza: entity.altezza,
            spessore_muro: entity.spessore_muro,
            riscaldamento: entity.riscaldamento.clone(),
            raffrescamento: entity.raffrescamento.clone(),
            illuminazione: entity.illuminazione.clone(),
            infissi: None,
        }
    }
}
