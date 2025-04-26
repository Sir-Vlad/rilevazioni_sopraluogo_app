use crate::dto::StanzaDTO;

#[derive(Clone)]
pub struct Stanza {
    pub id: Option<u64>,
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

impl Stanza {
    pub fn new(
        chiave: String,
        piano: String,
        id_spazio: String,
        stanza: String,
        destinazione_uso: String,
    ) -> Self {
        Stanza {
            id: None,
            chiave,
            piano,
            id_spazio,
            stanza,
            destinazione_uso,
            altezza: None,
            spessore_muro: None,
            riscaldamento: None,
            raffrescamento: None,
            illuminazione: None,
        }
    }
}

impl From<StanzaDTO> for Stanza {
    fn from(value: StanzaDTO) -> Self {
        Stanza {
            id: Some(value.id),
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