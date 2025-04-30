use crate::dto::EdificioDTO;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Edificio {
    pub(crate) chiave: String,
    pub(crate) fascicolo: String,
    pub(crate) indirizzo: String,
    pub(crate) anno_costruzione: Option<String>,
    pub(crate) anno_riqualificazione: Option<String>,
    pub(crate) note_riqualificazione: Option<String>,
    pub(crate) isolamento_tetto: Option<bool>,
    pub(crate) cappotto: Option<bool>,
}

impl Edificio {
    pub fn new(chiave: &str, fascicolo: &str, indirizzo: &str) -> Self {
        Edificio {
            chiave: chiave.to_string(),
            fascicolo: fascicolo.to_string(),
            indirizzo: indirizzo.to_string(),
            anno_costruzione: None,
            anno_riqualificazione: None,
            note_riqualificazione: None,
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
            note_riqualificazione: None,
            isolamento_tetto: value.isolamento_tetto,
            cappotto: value.cappotto,
        }
    }
}
