use crate::app_traits::{CreateTable, DaoTrait, GetAll, Insert};
use crate::dao::entity::AnnotazioneInfisso;
use crate::utils::AppError;

pub struct AnnotazioneInfissoDAO;

impl DaoTrait for AnnotazioneInfissoDAO {
    type Entity = AnnotazioneInfisso;
    type Error = AppError;
}

impl CreateTable for AnnotazioneInfissoDAO {}

impl GetAll for AnnotazioneInfissoDAO {}

impl Insert for AnnotazioneInfissoDAO {}
