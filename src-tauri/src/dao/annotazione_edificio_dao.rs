use crate::app_traits::{CreateTable, DaoTrait, GetAll, Insert};
use crate::dao::entity::AnnotazioneEdificio;
use crate::utils::AppError;

pub struct AnnotazioneEdificioDAO;

impl DaoTrait for AnnotazioneEdificioDAO {
    type Entity = AnnotazioneEdificio;
    type Error = AppError;
}

impl CreateTable for AnnotazioneEdificioDAO {}

impl GetAll for AnnotazioneEdificioDAO {}

impl Insert for AnnotazioneEdificioDAO {}
