#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Fotovoltaico {
    pub(crate) id: u64,
    pub(crate) id_edificio: String,
    pub(crate) potenza: u16,
    pub(crate) proprietario: String,
}

impl Fotovoltaico {
    pub fn new(id_edificio: &str, potenza: u16, proprietario: &str) -> Self {
        Self {
            id: 0,
            id_edificio: id_edificio.to_string(),
            potenza,
            proprietario: proprietario.to_string(),
        }
    }
}