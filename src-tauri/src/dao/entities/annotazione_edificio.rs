use crate::dto::AnnotazioneEdificioDTO;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct AnnotazioneEdificio {
    pub(crate) id: u64,
    pub(crate) id_edificio: String,
    pub(crate) content: String,
    pub(crate) _data: Option<String>,
}

impl From<AnnotazioneEdificioDTO> for AnnotazioneEdificio {
    fn from(value: AnnotazioneEdificioDTO) -> Self {
        Self {
            id: value.id,
            id_edificio: value.id_edificio,
            content: value.content,
            _data: None,
        }
    }
}
