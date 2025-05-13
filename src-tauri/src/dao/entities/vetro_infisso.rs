#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct VetroInfisso {
    pub(crate) _id: Option<u64>,
    pub(crate) vetro: String,
    pub(crate) efficienza_energetica: u8,
}

impl VetroInfisso {
    pub(crate) fn new(vetro: &str, efficienza_energetica: u8) -> Self {
        Self {
            _id: None,
            vetro: vetro.to_string(),
            efficienza_energetica,
        }
    }
}
