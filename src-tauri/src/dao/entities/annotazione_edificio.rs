#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct AnnotazioneEdificio {
    pub(crate) id: u64,
    pub(crate) id_edificio: String,
    pub(crate) content: String,
    pub(crate) data: String,
}
