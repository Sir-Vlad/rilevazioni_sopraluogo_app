use crate::service::Get;
use app_models::models::{Fotovoltaico, NewFotovoltaico, UpdateFotovoltaico};
use app_models::schema::fotovoltaico;
use app_utils::app_error::DomainError;
use app_utils::app_interface::dao_interface::crud_operations::{GetAll, Insert, Update};
use app_utils::app_interface::dao_interface::DAO;
use app_utils::app_interface::database_interface::PostgresPooled;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::{QueryDsl, RunQueryDsl};

pub struct FotovoltaicoDAO;

impl DAO for FotovoltaicoDAO {}

impl GetAll<Fotovoltaico> for FotovoltaicoDAO {
    type Output = Fotovoltaico;
    fn get_all(conn: &mut PostgresPooled) -> Result<Vec<Self::Output>, DomainError> {
        fotovoltaico::table.load(conn).map_err(DomainError::from)
    }
}

impl Get<Fotovoltaico, String> for FotovoltaicoDAO {
    type Output = Vec<Fotovoltaico>;

    fn get(conn: &mut PostgresPooled, id: String) -> Result<Self::Output, DomainError> {
        fotovoltaico::table
            .filter(fotovoltaico::edificio_id.eq(id))
            .get_results(conn)
            .map_err(|e| match e {
                Error::NotFound => DomainError::UtenzaNotFound,
                _ => DomainError::Unexpected(e),
            })
    }
}

impl Insert<NewFotovoltaico> for FotovoltaicoDAO {
    type Output = Fotovoltaico;
    fn insert(
        conn: &mut PostgresPooled,
        item: NewFotovoltaico,
    ) -> Result<Self::Output, DomainError> {
        diesel::insert_into(fotovoltaico::table)
            .values(&item)
            .get_result(conn)
            .map_err(|e| match e {
                Error::NotFound => DomainError::FotovoltaicoNotFound,
                Error::DatabaseError(kind, ..) => {
                    if matches!(kind, diesel::result::DatabaseErrorKind::UniqueViolation) {
                        DomainError::FotovoltaicoAlreadyExists
                    } else {
                        DomainError::from(e)
                    }
                }
                _ => DomainError::Unexpected(e),
            })
    }
}

impl Update<UpdateFotovoltaico, i32> for FotovoltaicoDAO {
    type Output = Fotovoltaico;
    fn update(
        conn: &mut PostgresPooled,
        id: i32,
        item: UpdateFotovoltaico,
    ) -> Result<Self::Output, DomainError> {
        diesel::update(fotovoltaico::table.find(id))
            .set(&item)
            .get_result(conn)
            .map_err(|e| match e {
                Error::NotFound => DomainError::FotovoltaicoNotFound,
                _ => DomainError::Unexpected(e),
            })
    }
}

#[cfg(test)]
mod test {
    use super::super::*;
    use app_models::models::{NewFotovoltaico, UpdateFotovoltaico};
    use app_utils::app_interface::dao_interface::crud_operations::{Insert, Update};
    use app_utils::test::create_postgres_pool;
    use diesel::RunQueryDsl;

    #[tokio::test]
    async fn test_insert_and_update_data() {
        let pool = create_postgres_pool().await;
        let mut conn = pool.get().unwrap();

        let insert_data = NewFotovoltaico {
            edificio_id: "9338-14".to_string(),
            potenza: 55f32,
            proprietario: "Ugo Ugolini".to_string(),
        };

        let result = match FotovoltaicoDAO::insert(&mut conn, insert_data.clone()) {
            Ok(res) => {
                assert_eq!(res.edificio_id, insert_data.edificio_id);
                println!("{res:#?}");
                res
            }
            Err(e) => {
                panic!("{}", e);
            }
        };

        let update_data = UpdateFotovoltaico {
            potenza: Some(85f32),
            proprietario: Some("Ugo Ugolini".to_string()),
        };
        match FotovoltaicoDAO::update(&mut conn, result.id, update_data.clone()) {
            Ok(res) => {
                assert_eq!(res.potenza, update_data.potenza.unwrap());
                println!("{res:#?}");
            }
            Err(e) => {
                panic!("{}", e);
            }
        }

        diesel::sql_query("DELETE FROM fotovoltaico WHERE ID = $1")
            .bind::<diesel::sql_types::Integer, _>(result.id)
            .execute(&mut conn)
            .unwrap();
    }
}
