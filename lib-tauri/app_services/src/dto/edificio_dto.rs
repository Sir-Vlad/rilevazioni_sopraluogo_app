use app_models::models::{Edificio, NewEdificio, UpdateEdificio};
use serde::{Deserialize, Serialize};
use app_interface::dto_interface::DTO;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EdificioDTO {
    pub chiave: String,
    pub fascicolo: i32,
    pub indirizzo: String,
    pub anno_costruzione: Option<i32>,
    pub anno_riqualificazione: Option<i32>,
    pub note_riqualificazione: Option<String>,
    pub isolamento_tetto: bool,
    pub cappotto: bool,
}

impl DTO for EdificioDTO {}

impl From<&Edificio> for EdificioDTO {
    fn from(value: &Edificio) -> Self {
        EdificioDTO {
            chiave: value.chiave.to_string(),
            fascicolo: value.fascicolo,
            indirizzo: value.indirizzo.to_string(),
            anno_costruzione: value.anno_costruzione.clone(),
            anno_riqualificazione: value.anno_riqualificazione.clone(),
            note_riqualificazione: value.note_riqualificazione.clone(),
            isolamento_tetto: value.isolamento_tetto,
            cappotto: value.cappotto,
        }
    }
}

impl From<EdificioDTO> for UpdateEdificio {
    fn from(value: EdificioDTO) -> Self {
        UpdateEdificio {
            anno_costruzione: value.anno_costruzione,
            anno_riqualificazione: value.anno_riqualificazione,
            note_riqualificazione: value.note_riqualificazione,
            isolamento_tetto: Some(value.isolamento_tetto),
            cappotto: Some(value.isolamento_tetto),
        }
    }
}

impl From<EdificioDTO> for NewEdificio {
    fn from(value: EdificioDTO) -> Self {
        NewEdificio {
            chiave: value.chiave,
            fascicolo: value.fascicolo,
            indirizzo: value.indirizzo,
        }
    }
}
