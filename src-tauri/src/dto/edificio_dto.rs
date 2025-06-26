use crate::app_traits::{DtoTrait, FromEntity};
use crate::entities::Edificio;
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

impl DtoTrait for EdificioDTO {
    type EntityLinked = Edificio;
}

impl FromEntity for EdificioDTO {
    fn from_entity(entity: <Self as DtoTrait>::EntityLinked) -> Self {
        Self {
            chiave: entity.chiave.to_string(),
            fascicolo: entity.fascicolo.to_string(),
            indirizzo: entity.indirizzo.to_string(),
            anno_costruzione: entity.anno_costruzione.clone(),
            anno_riqualificazione: entity.anno_riqualificazione.clone(),
            note_riqualificazione: entity.note_riqualificazione.clone(),
            isolamento_tetto: entity.isolamento_tetto,
            cappotto: entity.cappotto,
        }
    }
}
