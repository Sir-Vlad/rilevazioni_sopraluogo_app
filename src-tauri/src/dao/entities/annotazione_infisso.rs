use crate::dto::AnnotazioneInfissoDTO;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct AnnotazioneInfisso {
    pub(crate) id: u64,
    pub(crate) id_infisso: String,
    pub(crate) content: String,
    pub(crate) _data: Option<String>,
}

impl From<AnnotazioneInfissoDTO> for AnnotazioneInfisso {
    fn from(value: AnnotazioneInfissoDTO) -> Self {
        Self {
            id: value.id,
            id_infisso: value.id_infisso,
            content: value.content,
            _data: None,
        }
    }
}
