use crate::dto::StanzaDTO;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Stanza {
    pub(crate) id: Option<u64>,
    pub(crate) chiave: String,
    pub(crate) piano: String,
    pub(crate) id_spazio: String,
    pub(crate) stanza: String,
    pub(crate) destinazione_uso: String,
    pub(crate) altezza: Option<u16>,
    pub(crate) spessore_muro: Option<u8>,
    pub(crate) riscaldamento: Option<String>,
    pub(crate) raffrescamento: Option<String>,
    pub(crate) illuminazione: Option<String>,
}

impl Stanza {
    pub fn new(
        chiave: &str,
        piano: &str,
        id_spazio: &str,
        stanza: &str,
        destinazione_uso: &str,
    ) -> Self {
        Stanza {
            id: None,
            chiave: chiave.to_string(),
            piano: piano.to_string(),
            id_spazio: id_spazio.to_string(),
            stanza: stanza.to_string(),
            destinazione_uso: destinazione_uso.to_string(),
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
