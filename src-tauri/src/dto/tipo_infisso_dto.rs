use crate::app_traits::DtoTrait;
use crate::entities::TipoInfisso;
use serde::Serialize;

#[derive(Serialize)]
pub struct TipoInfissiDTO {
    pub(crate) nome: String,
}

impl DtoTrait for TipoInfissiDTO {
    type EntityLinked = TipoInfisso;
}
