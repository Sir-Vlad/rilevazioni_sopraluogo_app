use crate::dto::AnnotazioneStanzaDTO;

#[cfg_attr(test, derive(Debug, PartialEq, Clone))]
pub struct AnnotazioneStanza {
    pub(crate) id: u64,
    pub(crate) id_stanza: u64,
    pub(crate) content: String,
    pub(crate) _data: Option<String>,
}

impl From<AnnotazioneStanzaDTO> for AnnotazioneStanza {
    fn from(value: AnnotazioneStanzaDTO) -> Self {
        Self {
            id: value.id,
            id_stanza: value.id_stanza,
            content: value.content,
            _data: None,
        }
    }
}
