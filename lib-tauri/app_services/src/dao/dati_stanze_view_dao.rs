use app_utils::app_error::DomainError;
use app_utils::app_interface::dao_interface::crud_operations::{Get, GetAll};
use app_utils::app_interface::dao_interface::DAO;
use app_utils::app_interface::database_interface::PostgresPooled;
use app_models::models::DatiStanza;
use diesel::RunQueryDsl;

pub struct DatiStanzeViewDAO;

impl DAO for DatiStanzeViewDAO {}

impl Get<DatiStanza, i32> for DatiStanzeViewDAO {
    type Output = Vec<DatiStanza>;
    fn get(conn: &mut PostgresPooled, fascicolo: i32) -> Result<Self::Output, DomainError> {
        let result = diesel::sql_query("SELECT * FROM V_DATI_STANZE WHERE fascicolo = $1")
            .bind::<diesel::sql_types::Integer, _>(fascicolo)
            .load::<DatiStanza>(conn)
            .map_err(DomainError::from)?;

        Ok(result)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use app_utils::test::create_postgres_pool;

    #[tokio::test]
    async fn test_retrieve() {
        let pool = create_postgres_pool().await;
        let mut conn = pool.get().unwrap();

        match DatiStanzeViewDAO::get(&mut conn, 1912) {
            Ok(dati_stanze) => {
                println!("{dati_stanze:?}");
                assert!(!dati_stanze.is_empty())
            }
            Err(e) => panic!("{e:?}", )
        }
    }
}