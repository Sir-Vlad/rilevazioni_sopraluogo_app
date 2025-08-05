use crate::dto::EdificioDTO;
use crate::utils::ToList;
use std::any::Any;

#[cfg_attr(test, derive(Debug, PartialEq, Clone))]
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

    pub fn get_fields() -> Vec<String> {
        vec![
            "chiave".to_string(),
            "fascicolo".to_string(),
            "indirizzo".to_string(),
            "anno_costruzione".to_string(),
            "anno_riqualificazione".to_string(),
            "note_riqualificazione".to_string(),
            "isolamento_tetto".to_string(),
            "cappotto".to_string(),
        ]
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

impl ToList for Edificio {
    fn to_list(&self) -> Vec<Box<dyn Any>> {
        vec![
            Box::new(self.chiave.clone()),
            Box::new(self.fascicolo.clone()),
            Box::new(self.indirizzo.clone()),
            Box::new(self.anno_costruzione.clone()),
            Box::new(self.anno_riqualificazione.clone()),
            Box::new(self.note_riqualificazione.clone()),
            Box::new(self.isolamento_tetto),
            Box::new(self.cappotto),
        ]
    }
}
