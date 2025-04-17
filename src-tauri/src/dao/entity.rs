use crate::dto::{EdificioDTO, InfissoDto, StanzaDto};

#[derive(Debug)]
pub struct Edificio {
    pub chiave: String,
    pub fascicolo: String,
    pub indirizzo: String,
    pub anno_costruzione: Option<String>,
    pub anno_riqualificazione: Option<String>,
    pub isolamento_tetto: Option<bool>,
    pub cappotto: Option<bool>,
}

impl Edificio {
    pub fn new(chiave: String, fascicolo: String, indirizzo: String) -> Self {
        Edificio{
            chiave,
            fascicolo,
            indirizzo,
            anno_costruzione: None,
            anno_riqualificazione: None,
            isolamento_tetto: None,
            cappotto: None,
        }
    }
}

impl From<EdificioDTO> for Edificio {
    fn from(value: EdificioDTO) -> Self {
        Edificio {
            chiave: value.chiave.to_string(),
            fascicolo: value.fascicolo.to_string(),
            indirizzo: value.indirizzo.to_string(),
            anno_costruzione: value.anno_costruzione.clone(),
            anno_riqualificazione: value.anno_riqualificazione.clone(),
            isolamento_tetto: value.isolamento_tetto,
            cappotto: value.cappotto,
        }
    }
}

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
