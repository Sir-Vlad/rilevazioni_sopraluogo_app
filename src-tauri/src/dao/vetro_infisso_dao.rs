use crate::app_traits::{CreateTable, DaoTrait, GetAll, Insert};
use crate::dao::entity::VetroInfisso;
use crate::utils::AppError;

pub struct VetroInfissoDAO;

impl DaoTrait for VetroInfissoDAO {
    type Entity = VetroInfisso;
    type Error = AppError;
}
impl CreateTable for VetroInfissoDAO {}
impl GetAll for VetroInfissoDAO {}
impl Insert for VetroInfissoDAO {}
