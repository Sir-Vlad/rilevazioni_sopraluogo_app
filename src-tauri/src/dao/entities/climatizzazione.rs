#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Climatizzazione {
    #[allow(dead_code)]
    pub(crate) id: u64,
    pub(crate) climatizzazione: String,
    pub(crate) efficienza_energetica: u8,
}
