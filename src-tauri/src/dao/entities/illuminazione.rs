#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Illuminazione {
    #[allow(dead_code)]
    pub(crate) id: u64,
    pub(crate) lampadina: String,
    pub(crate) efficienza_energetica: u8,
}