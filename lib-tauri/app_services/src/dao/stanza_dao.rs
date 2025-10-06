use app_models::{
    models::{NewStanza, Stanza, UpdateStanza},
    schema::stanza,
};
use app_utils::{
    app_error::DomainError,
    app_interface::{
        dao_interface::{
            DAO,
            crud_operations::{Get, Insert, Update},
        },
        database_interface::PostgresPooled,
    },
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, result::Error};

use crate::dao::utils::{EntityType::Stanza as StanzaType, map_error_for_entity};

pub struct StanzaDAO;

impl DAO for StanzaDAO {}

impl Get<Stanza, &str> for StanzaDAO {
    type Output = Vec<Stanza>;

    fn get(conn: &mut PostgresPooled, edificio: &str) -> Result<Self::Output, DomainError> {
        stanza::table
            .filter(stanza::edificio_id.eq(edificio))
            .get_results(conn)
            .map_err(|e| match e {
                Error::NotFound => DomainError::StanzaNotFound,
                _ => DomainError::Unexpected(e),
            })
    }
}

impl Insert<NewStanza<'_>> for StanzaDAO {
    type Output = Stanza;

    fn insert(conn: &mut PostgresPooled, item: NewStanza) -> Result<Self::Output, DomainError> {
        diesel::insert_into(stanza::table)
            .values(&item)
            .get_result(conn)
            .map_err(|e| map_error_for_entity(e, StanzaType))
    }
}

impl Insert<Vec<NewStanza<'_>>> for StanzaDAO {
    type Output = Vec<Stanza>;

    fn insert(
        conn: &mut PostgresPooled,
        item: Vec<NewStanza>,
    ) -> Result<Self::Output, DomainError> {
        diesel::insert_into(stanza::table)
            .values(&item)
            .get_results(conn)
            .map_err(|e| map_error_for_entity(e, StanzaType))
    }
}

impl Update<UpdateStanza<'_>, i32> for StanzaDAO {
    type Output = Stanza;

    fn update(
        conn: &mut PostgresPooled,
        id: i32,
        item: UpdateStanza,
    ) -> Result<Self::Output, DomainError> {
        diesel::update(stanza::table.find(id))
            .set(&item)
            .get_result(conn)
            .map_err(|e| match e {
                Error::NotFound => DomainError::StanzaNotFound,
                _ => DomainError::Unexpected(e),
            })
    }
}

#[cfg(test)]
mod tests {}
