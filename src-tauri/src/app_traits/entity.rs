use crate::app_traits::sql_params::SqlParams;
use crate::database::QueryBuilderError;
use rusqlite::Row;

pub trait FromRow {
    fn from_row(row: &Row) -> Result<Self, rusqlite::Error>
    where
        Self: Sized;
}

pub trait EntityTrait: FromRow {
    type PrimaryKey: Clone + PartialEq + SqlParams;

    fn table_name() -> String;
    fn sql_create_table() -> String;
}

pub trait ToRetrieve: EntityTrait {
    fn to_retrieve() -> String;
}

pub trait ToRetrieveAll: EntityTrait {
    #[inline]
    fn to_retrieve_all() -> String {
        format!("SELECT * FROM {}", Self::table_name())
    }
}

pub trait ToInsert: EntityTrait {
    fn to_insert() -> String;
    fn to_insert_params(&self) -> Vec<&dyn SqlParams>;
}

pub trait ToUpdate: EntityTrait {
    fn to_update() -> String;
    fn to_build_update(
        &self,
    ) -> Result<Option<(String, Vec<Box<&dyn SqlParams>>)>, QueryBuilderError> {
        Ok(None)
    }
    fn to_update_params(&self) -> Vec<Box<&dyn SqlParams>>;
}

pub trait ToDelete: EntityTrait {
    fn to_delete() -> String;
}

pub trait ToCrud: ToRetrieve + ToRetrieveAll + ToInsert + ToUpdate + ToDelete {}
impl<T> ToCrud for T where T: ToRetrieve + ToRetrieveAll + ToInsert + ToUpdate + ToDelete {}
