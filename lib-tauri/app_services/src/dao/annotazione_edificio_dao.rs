use crate::dao::utils::map_error_annotazione;
use app_utils::app_error::DomainError;
use app_utils::app_interface::dao_interface::crud_operations::{GetAll, Insert};
use app_utils::app_interface::dao_interface::DAO;
use app_utils::app_interface::database_interface::PostgresPooled;
use app_models::models::{AnnotazioneEdificio, NewAnnotazioneEdificio};
use app_models::schema::annotazione_edificio;
use diesel::result::Error;
use diesel::RunQueryDsl;

pub struct AnnotazioneEdificioDAO;

impl DAO for AnnotazioneEdificioDAO {}

impl GetAll<AnnotazioneEdificio> for AnnotazioneEdificioDAO {
    type Output = AnnotazioneEdificio;
    fn get_all(conn: &mut PostgresPooled) -> Result<Vec<Self::Output>, DomainError> {
        annotazione_edificio::table.load(conn).map_err(|e| match e {
            Error::NotFound => {
                DomainError::AnnotazioneNotFound
            }
            _ => DomainError::Unexpected(e),
        })
    }
}

impl Insert<NewAnnotazioneEdificio> for AnnotazioneEdificioDAO {
    type Output = AnnotazioneEdificio;
    fn insert(
        conn: &mut PostgresPooled,
        item: NewAnnotazioneEdificio,
    ) -> Result<Self::Output, DomainError> {
        diesel::insert_into(annotazione_edificio::table)
            .values(&item)
            .get_result(conn)
            .map_err(map_error_annotazione)
    }
}
