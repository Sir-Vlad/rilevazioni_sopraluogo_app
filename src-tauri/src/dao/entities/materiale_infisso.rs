#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct MaterialeInfisso {
    #[allow(dead_code)]
    pub(crate) id: u64,
    pub(crate) materiale: String,
    pub(crate) efficienza_energetica: u8,
}
