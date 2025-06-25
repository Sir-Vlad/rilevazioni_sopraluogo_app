use crate::app_traits::{CreateTable, DaoTrait, GetAll, Insert};
use crate::entities::Illuminazione;
use crate::utils::AppError;

pub struct IlluminazioneDAO;

impl DaoTrait for IlluminazioneDAO {
    type Entity = Illuminazione;
    type Error = AppError;
}

impl CreateTable for IlluminazioneDAO {}
impl GetAll for IlluminazioneDAO {}
impl Insert for IlluminazioneDAO {}
