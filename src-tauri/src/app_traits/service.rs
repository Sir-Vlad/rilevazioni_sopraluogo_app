use crate::app_traits::DtoTrait;
use crate::db::Database;
use crate::utils::AppError;
use tauri::State;

pub trait CreateService<T>
where
    T: DtoTrait,
{
    fn create(db: State<'_, Database>, item: T) -> Result<T, AppError>;
}

#[allow(dead_code)]
pub trait RetrieveOneService<T, K>
where
    T: DtoTrait,
{
    fn retrieve_one(db: State<'_, Database>, id: K) -> Result<T, AppError>;
}

pub trait RetrieveManyService<T>
where
    T: DtoTrait,
{
    fn retrieve_many(db: State<'_, Database>) -> Result<Vec<T>, AppError>;
}

pub trait UpdateService<T>
where
    T: DtoTrait,
{
    fn update(db: State<'_, Database>, item: T) -> Result<T, AppError>;
}

#[allow(dead_code)]
pub trait DeleteService<T, K>
where
    T: DtoTrait,
{
    fn delete(db: State<'_, Database>, id: K) -> Result<bool, AppError>;
}
