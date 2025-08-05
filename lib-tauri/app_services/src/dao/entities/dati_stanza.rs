use serde::Serialize;
use std::any::Any;
use crate::utils::ToList;

#[derive(Debug, Serialize)]
pub struct DatiStanza {
    pub(crate) id: u64,
    pub(crate) fascicolo: String,
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
    pub(crate) mq_infissi: Option<f32>,
    pub(crate) materiale: Option<String>,
    pub(crate) vetro: Option<String>,
}

impl DatiStanza {
    pub fn get_fields() -> Vec<&'static str> {
        vec![
            "id",
            "fascicolo",
            "chiave",
            "piano",
            "id_spazio",
            "stanza",
            "destinazione_uso",
            "altezza",
            "spessore_muro",
            "riscaldamento",
            "raffrescamento",
            "illuminazione",
            "mq_infissi",
            "materiale",
            "vetro",
        ]
    }
}

impl ToList for DatiStanza{
    fn to_list(&self) -> Vec<Box<dyn Any>> {
        vec![Box::new(self.id), 
             Box::new(self.fascicolo.clone()),
             Box::new(self.chiave.clone()),
             Box::new(self.piano.clone()),
             Box::new(self.id_spazio.clone()),
             Box::new(self.stanza.clone()),
             Box::new(self.destinazione_uso.clone()),
             Box::new(self.altezza),
             Box::new(self.spessore_muro),
             Box::new(self.riscaldamento.clone()),
             Box::new(self.raffrescamento.clone()),
             Box::new(self.illuminazione.clone()),
             Box::new(self.mq_infissi),
             Box::new(self.materiale.clone()),
             Box::new(self.vetro.clone())
        ]
    }
}
