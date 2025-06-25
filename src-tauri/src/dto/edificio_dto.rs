use crate::entities::Edificio;
use crate::dto::DTO;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EdificioDTO {
    pub chiave: String,
    pub fascicolo: String,
    pub indirizzo: String,
    pub anno_costruzione: Option<String>,
    pub anno_riqualificazione: Option<String>,
    pub note_riqualificazione: Option<String>,
    pub isolamento_tetto: Option<bool>,
    pub cappotto: Option<bool>,
}

impl DTO for EdificioDTO {}

impl From<&Edificio> for EdificioDTO {
    fn from(value: &Edificio) -> Self {
        EdificioDTO {
            chiave: value.chiave.to_string(),
            fascicolo: value.fascicolo.to_string(),
            indirizzo: value.indirizzo.to_string(),
            anno_costruzione: value.anno_costruzione.clone(),
            anno_riqualificazione: value.anno_riqualificazione.clone(),
            note_riqualificazione: value.note_riqualificazione.clone(),
            isolamento_tetto: value.isolamento_tetto,
            cappotto: value.cappotto,
        }
    }
}
