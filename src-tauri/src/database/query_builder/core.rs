use crate::database::query_builder::{
    DeleteQueryBuilder, InsertQueryBuilder, SelectQueryBuilder, UpdateQueryBuilder,
};

pub struct QueryBuilder;

impl QueryBuilder {
    pub fn select() -> SelectQueryBuilder {
        SelectQueryBuilder::new()
    }

    pub fn insert() -> InsertQueryBuilder {
        InsertQueryBuilder::new()
    }

    pub fn update() -> UpdateQueryBuilder {
        UpdateQueryBuilder::new()
    }

    pub fn delete() -> DeleteQueryBuilder {
        DeleteQueryBuilder::new()
    }
}
