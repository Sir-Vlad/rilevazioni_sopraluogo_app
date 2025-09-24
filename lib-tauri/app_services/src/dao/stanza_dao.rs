use app_models::models::{NewStanza, Stanza, UpdateStanza};
use app_models::schema::stanza;
use app_utils::app_error::DomainError;
use app_utils::app_interface::dao_interface::crud_operations::{Get, Insert, Update};
use app_utils::app_interface::dao_interface::DAO;
use app_utils::app_interface::database_interface::PostgresPooled;
use diesel::result::Error;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use crate::dao::utils::EntityType::Stanza as StanzaType;
use crate::dao::utils::map_error_for_entity;

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

impl Insert<NewStanza> for StanzaDAO {
    type Output = Stanza;
    fn insert(conn: &mut PostgresPooled, item: NewStanza) -> Result<Self::Output, DomainError> {
        diesel::insert_into(stanza::table)
            .values(&item)
            .get_result(conn)
            .map_err(|e| map_error_for_entity(e, StanzaType))
    }
}

impl Insert<Vec<NewStanza>> for StanzaDAO {
    type Output = Vec<Stanza>;

    fn insert(conn: &mut PostgresPooled, item: Vec<NewStanza>) -> Result<Self::Output, DomainError> {
        diesel::insert_into(stanza::table)
            .values(&item)
            .get_results(conn)
            .map_err(|e| map_error_for_entity(e, StanzaType))
    }
}

impl Update<UpdateStanza, i32> for StanzaDAO {
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
