#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct AnnotazioneStanza {
    pub(crate) id: u64,
    pub(crate) id_stanza: u64,
    pub(crate) content: String,
    pub(crate) data: String,
}
