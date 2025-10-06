use app_models::{models::Climatizzazione, schema::climatizzazione};
use app_utils::{
    app_error::DomainError,
    app_interface::{
        dao_interface::{
            DAO,
            crud_operations::{GetAll, Insert},
        },
        database_interface::PostgresPooled,
    },
};
use diesel::{RunQueryDsl, result::Error};

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

    fn insert(
        conn: &mut PostgresPooled,
        item: Climatizzazione,
    ) -> Result<Self::Output, DomainError> {
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
    use app_utils::{
        app_interface::dao_interface::crud_operations::GetAll, test::create_postgres_pool,
    };

    use crate::dao::climatizzazione_dao::ClimatizzazioneDAO;

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
