#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct MaterialeInfisso {
    pub(crate) _id: Option<u64>,
    pub(crate) materiale: String,
    pub(crate) efficienza_energetica: u8,
}

impl MaterialeInfisso {
    pub(crate) fn new(materiale: &str, efficienza_energetica: u8) -> Self {
        Self {
            _id: None,
            materiale: materiale.to_string(),
            efficienza_energetica,
        }
    }
}
