use app_models::models::Climatizzazione;
use app_models::schema::climatizzazione;
use app_utils::app_error::DomainError;
use app_utils::app_interface::dao_interface::crud_operations::{GetAll, Insert};
use app_utils::app_interface::dao_interface::DAO;
use app_utils::app_interface::database_interface::PostgresPooled;
use diesel::result::Error;
use diesel::RunQueryDsl;

pub struct ClimatizzazioneDAO;

impl DAO for ClimatizzazioneDAO {}

impl GetAll<Climatizzazione> for ClimatizzazioneDAO {
    type Output = Climatizzazione;
    fn get_all(conn: &mut PostgresPooled) -> Result<Vec<Self::Output>, DomainError> {
        climatizzazione::table
            .load::<Climatizzazione>(conn)
            .map_err(|e| match e {
                Error::NotFound => DomainError::AnnotazioneNotFound,
                _ => DomainError::Unexpected(e),
            })
    }
}

impl Insert<Climatizzazione> for ClimatizzazioneDAO {
    type Output = Climatizzazione;

    fn insert(conn: &mut PostgresPooled, item: Climatizzazione) -> Result<Self::Output, DomainError> {
        diesel::insert_into(climatizzazione::table)
            .values(&item)
            .get_result(conn)
            .map_err(|e| match e {
                Error::NotFound => DomainError::ClimatizzazioneNotFound,
                _ => DomainError::Unexpected(e),
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::dao::climatizzazione_dao::ClimatizzazioneDAO;
    use app_utils::app_interface::dao_interface::crud_operations::GetAll;
    use app_utils::test::create_postgres_pool;

    #[tokio::test]
    async fn get_all() {
        let pool = create_postgres_pool().await;
        let mut conn = pool.get().unwrap();

        match ClimatizzazioneDAO::get_all(&mut conn) {
            Ok(data) => assert_eq!(data.len(), 7),
            Err(e) => panic!("Errore: {e}"),
        }
    }
}
