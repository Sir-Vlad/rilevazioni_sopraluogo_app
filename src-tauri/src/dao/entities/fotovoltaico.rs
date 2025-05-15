use crate::dto::FotovoltaicoDTO;
use crate::utils::ToList;
use std::any::Any;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Fotovoltaico {
    pub(crate) id: u64,
    pub(crate) id_edificio: String,
    pub(crate) potenza: f32,
    pub(crate) proprietario: String,
}

impl Fotovoltaico {
    pub fn new(id_edificio: &str, potenza: f32, proprietario: &str) -> Self {
        Self {
            id: 0,
            id_edificio: id_edificio.to_string(),
            potenza,
            proprietario: proprietario.to_string(),
        }
    }

    pub fn get_fields() -> Vec<String> {
        vec![
            "id".to_string(),
            "id_edificio".to_string(),
            "potenza".to_string(),
            "proprietario".to_string(),
        ]
    }
}

impl From<FotovoltaicoDTO> for Fotovoltaico {
    fn from(value: FotovoltaicoDTO) -> Self {
        Self {
            id: value.id.unwrap_or(0),
            id_edificio: value.id_edificio,
            potenza: value.potenza,
            proprietario: value.proprietario,
        }
    }
}

impl ToList for Fotovoltaico {
    fn to_list(&self) -> Vec<Box<dyn Any>> {
        vec![
            Box::new(self.id),
            Box::new(self.id_edificio.clone()),
            Box::new(self.potenza),
            Box::new(self.proprietario.clone()),
        ]
    }
}
