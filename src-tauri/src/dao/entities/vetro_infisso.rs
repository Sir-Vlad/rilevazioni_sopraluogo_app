#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct VetroInfisso {
    #[allow(dead_code)]
    pub(crate) id: u64,
    pub(crate) vetro: String,
    pub(crate) efficienza_energetica: u8,
}
