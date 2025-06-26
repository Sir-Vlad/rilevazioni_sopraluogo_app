use crate::app_traits::EntityTrait;

pub trait DtoTrait {
    type EntityLinked;
}

pub trait FromDto: EntityTrait + Sized {
    type Dto: DtoTrait + Sized;
    fn from_dto(dto: Self::Dto) -> Self;
}

pub trait FromEntity: DtoTrait + Sized {
    fn from_entity(entity: <Self as DtoTrait>::EntityLinked) -> Self;
}

pub trait ConvertibleDto: DtoTrait + Sized {
    fn into_entity(self) -> Self::EntityLinked
    where
        Self::EntityLinked: FromDto<Dto = Self>,
    {
        Self::EntityLinked::from_dto(self)
    }
}

pub trait ConvertibleEntity: EntityTrait + Sized {
    fn into_dto<D: FromEntity<EntityLinked = Self>>(self) -> D {
        D::from_entity(self)
    }
}

impl<T> ConvertibleDto for T
where
    T: FromEntity,
    T::EntityLinked: FromDto<Dto = T>,
{
}

impl<T> ConvertibleEntity for T where T: EntityTrait {}
