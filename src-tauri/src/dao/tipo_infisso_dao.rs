use crate::app_traits::{CreateTable, DaoTrait, GetAll, Insert};
use crate::dao::entity::TipoInfisso;
use crate::utils::AppError;

pub struct TipoInfissoDAO;

impl DaoTrait for TipoInfissoDAO {
    type Entity = TipoInfisso;
    type Error = AppError;
}
impl CreateTable for TipoInfissoDAO {}
impl GetAll for TipoInfissoDAO {}
impl Insert for TipoInfissoDAO {}
