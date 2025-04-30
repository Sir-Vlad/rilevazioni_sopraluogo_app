#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct CommentoInfisso {
    pub(crate) id: u64,
    pub(crate) id_infisso: String,
    pub(crate) content: String,
    pub(crate) data: String,
}
