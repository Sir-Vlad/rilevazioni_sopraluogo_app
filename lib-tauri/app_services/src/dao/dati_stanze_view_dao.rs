use app_models::models::DatiStanza;
use app_utils::{
    app_error::DomainError,
    app_interface::{
        dao_interface::{DAO, crud_operations::Get},
        database_interface::PostgresPooled,
    },
};
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
    use app_utils::test::create_postgres_pool;

    use super::*;

    #[tokio::test]
    async fn test_retrieve() {
        let pool = create_postgres_pool().await;
        let mut conn = pool.get().unwrap();

        match DatiStanzeViewDAO::get(&mut conn, 1912) {
            Ok(dati_stanze) => {
                println!("{dati_stanze:?}");
                assert!(!dati_stanze.is_empty())
            }
            Err(e) => panic!("{e:?}",),
        }
    }
}
