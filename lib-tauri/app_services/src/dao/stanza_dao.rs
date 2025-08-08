use app_error::DomainError;
use app_interface::dao_interface::crud_operations::{GetAll, Insert, Update};
use app_interface::dao_interface::DAO;
use app_interface::database_interface::PostgresPooled;
use app_models::models::{NewStanza, Stanza, UpdateStanza};
use app_models::schema::stanza;
use diesel::result::Error;
use diesel::{QueryDsl, RunQueryDsl};

pub struct StanzaDAO;

impl DAO for StanzaDAO {
}

impl GetAll<Stanza> for StanzaDAO {
    type Output = Stanza;
    fn get_all(conn: &mut PostgresPooled) -> Result<Vec<Self::Output>, DomainError> {
        stanza::table.load(conn).map_err(DomainError::from)
    }
}

impl Insert<NewStanza> for StanzaDAO {
    type Output = Stanza;
    fn insert(conn: &mut PostgresPooled, item: NewStanza) -> Result<Self::Output, DomainError> {
        diesel::insert_into(stanza::table)
            .values(&item)
            .get_result(conn)
            .map_err(|e| match e {
                Error::NotFound => DomainError::StanzaNotFound,
                Error::DatabaseError(kind, ..) => {
                    if matches!(kind, diesel::result::DatabaseErrorKind::UniqueViolation) {
                        DomainError::StanzaAlreadyExists
                    } else {
                        DomainError::from(e)
                    }
                }
                _ => DomainError::Unexpected(e),
            })
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
mod tests {

}
