#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct MaterialeInfisso {
    #[allow(dead_code)]
    pub(crate) id: u64,
    pub(crate) materiale: String,
    pub(crate) efficienza_energetica: u8,
}

impl MaterialeInfisso {
    pub(crate) fn new(materiale: &str, efficienza_energetica: u8) -> Self {
        Self {
            id: 0,
            materiale: materiale.to_string(),
            efficienza_energetica,
        }
    }
}
