use crate::dto::{InfissoDto, StanzaDto};

#[derive(Debug)]
pub struct Infisso {
    pub id: String,
    pub tipo: String,
    pub altezza: i32,
    pub larghezza: i32,
    pub materiale: String,
    pub vetro: String,
}

impl From<&InfissoDto> for Infisso {
    fn from(infisso: &InfissoDto) -> Self {
        Infisso {
            id: infisso.id.clone(),
            tipo: infisso.tipo.clone(),
            altezza: infisso.altezza,
            larghezza: infisso.larghezza,
            materiale: infisso.materiale.clone(),
            vetro: infisso.vetro.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Stanza {
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
}

impl From<&StanzaDto> for Stanza {
    fn from(value: &StanzaDto) -> Self {
        Stanza {
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
        }
    }
}

#[derive(Debug)]
pub struct MaterialeInfisso {
    pub id: u64,
    pub materiale: String,
    pub efficienza_energetica: i8,
}

#[derive(Debug)]
pub struct VetroInfisso {
    pub id: u64,
    pub vetro: String,
    pub efficienza_energetica: i8,
}

#[derive(Debug)]
pub struct Illuminazione {
    pub id: u64,
    pub lampadina: String,
    pub efficienza_energetica: i8,
}

#[derive(Debug, PartialEq)]
pub struct Climatizzazione {
    pub id: u64,
    pub climatizzazione: String,
    pub efficienza_energetica: i8,
}
