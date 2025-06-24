use crate::app_traits::{CreateTable, DaoTrait, GetAll, Insert};
use crate::dao::entity::MaterialeInfisso;
use crate::utils::AppError;

pub struct MaterialeInfissoDAO;

impl DaoTrait for MaterialeInfissoDAO {
    type Entity = MaterialeInfisso;
    type Error = AppError;
}
impl CreateTable for MaterialeInfissoDAO {}
impl GetAll for MaterialeInfissoDAO {}
impl Insert for MaterialeInfissoDAO {}