use app_utils::app_error::DomainError;
use app_utils::app_interface::dao_interface::crud_operations::{GetAll, Insert};
use app_utils::app_interface::dao_interface::DAO;
use app_utils::app_interface::database_interface::PostgresPooled;
use app_models::models::Climatizzazione;
use app_models::schema::climatizzazione;
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
                Error::NotFound => {
                    DomainError::AnnotazioneNotFound
                }
                _ => DomainError::Unexpected(e),
            })
    }
}
//
// impl Insert<NewClima> for ClimatizzazioneDAO {
//     fn insert(conn: &mut PostgresPooled, item: Climatizzazione) -> Result<Self::Output, DomainError> {
//         diesel::insert_into(climatizzazione::table)
//             .values()
//     }
//     // fn insert<C: DatabaseConnection>(
//     //     conn: &C,
//     //     item: Climatizzazione,
//     // ) -> Result<Climatizzazione, AppError> {
//     //     let builder = QueryBuilder::insert()
//     //         .table(Self::table_name())
//     //         .columns(vec!["CLIMATIZZAZIONE", "EFFICIENZA_ENERGETICA"])
//     //         .values(vec![
//     //             item.climatizzazione.clone().into(),
//     //             item.efficienza_energetica.into(),
//     //         ])
//     //         .returning("ID");
//     //     let (query, param) = builder.build()?;
//     //     let mut stmt = conn.prepare(query.as_str())?;
//     //     let mut res = stmt.query_map(rusqlite::params_from_iter(convert_param(param)), |row| {
//     //         row.get::<_, u64>(0)
//     //     })?;
//     //     let id = res.next().unwrap()?;
//     //     info!(
//     //         "Nuovo tipo di climatizzazione inserito con ID: {}",
//     //         item.climatizzazione
//     //     );
//     //     Ok(Climatizzazione {
//     //         _id: Some(id),
//     //         ..item
//     //     })
//     // }
// }

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
