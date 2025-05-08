#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct VetroInfisso {
    #[allow(dead_code)]
    pub(crate) id: u64,
    pub(crate) vetro: String,
    pub(crate) efficienza_energetica: u8,
}

impl VetroInfisso {
    pub(crate) fn new(vetro: &str, efficienza_energetica: u8) -> Self {
        Self{
            id: 0,
            vetro: vetro.to_string(),
            efficienza_energetica,
        }
    }
}
