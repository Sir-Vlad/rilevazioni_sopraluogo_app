use app_utils::app_error::DomainError;
use app_utils::app_interface::dao_interface::crud_operations::{GetAll, Insert, Update};
use app_utils::app_interface::dao_interface::DAO;
use app_utils::app_interface::database_interface::PostgresPooled;
use app_models::models::{Infisso, NewInfisso, UpdateInfisso};
use app_models::schema::infisso;
use diesel::result::Error;
use diesel::{QueryDsl, RunQueryDsl};

pub struct InfissoDAO;

impl DAO for InfissoDAO {}

impl GetAll<Infisso> for InfissoDAO {
    type Output = Infisso;
    fn get_all(conn: &mut PostgresPooled) -> Result<Vec<Self::Output>, DomainError> {
        infisso::table.load(conn).map_err(DomainError::from)
    }
}

impl Insert<NewInfisso> for InfissoDAO {
    type Output = Infisso;
    fn insert(conn: &mut PostgresPooled, item: NewInfisso) -> Result<Self::Output, DomainError> {
        diesel::insert_into(infisso::table)
            .values(&item)
            .get_result(conn)
            .map_err(|e| match e {
                Error::NotFound => DomainError::InfissoNotFound,
                Error::DatabaseError(kind, ref db_info) => {
                    if matches!(kind, diesel::result::DatabaseErrorKind::UniqueViolation) {
                        DomainError::InfissoAlreadyExists
                    } else {
                        DomainError::from(e)
                    }
                }
                _ => DomainError::Unexpected(e),
            })
    }
}

impl Update<UpdateInfisso, (String, String)> for InfissoDAO {
    type Output = Infisso;

    /// id -> (infisso, edificio)
    fn update(
        conn: &mut PostgresPooled,
        id: (String, String),
        item: UpdateInfisso,
    ) -> Result<Self::Output, DomainError> {
        let id_infisso = id.0;
        let id_edificio = id.1;

        diesel::update(infisso::table.find((id_infisso, id_edificio)))
            .set(&item)
            .get_result(conn)
            .map_err(|e| match e {
                Error::NotFound => DomainError::InfissoNotFound,
                _ => DomainError::Unexpected(e),
            })
    }
}
