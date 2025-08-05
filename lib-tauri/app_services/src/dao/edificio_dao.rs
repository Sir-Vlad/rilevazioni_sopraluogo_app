use app_database::database::database_manager::PostgresPooled;
use app_error::DomainError;
use app_interface::dao_interface::crud_operations::{Get, GetAll, Insert, Update};
use app_interface::dao_interface::DAO;
use app_models::models::{Edificio, NewEdificio, UpdateEdificio};
use app_models::schema::edificio;
use diesel::result::Error;
use diesel::{QueryDsl, RunQueryDsl};

pub struct EdificioDAO;

impl DAO for EdificioDAO {
    type Item = Edificio;
}

impl GetAll<Edificio> for EdificioDAO {
    fn get_all(conn: &mut PostgresPooled) -> Result<Vec<Self::Item>, DomainError> {
        edificio::table.load::<Edificio>(conn).map_err(|e| match e {
            Error::NotFound => DomainError::EdificioNotFound("Not find edificio".to_string()),
            _ => DomainError::Unexpected(e),
        })
    }
}

impl Get<Edificio, String> for EdificioDAO {
    fn get(conn: &mut PostgresPooled, id: String) -> Result<Self::Item, DomainError> {
        edificio::table
            .find(id)
            .first::<Edificio>(conn)
            .map_err(|e| match e {
                Error::NotFound => DomainError::EdificioNotFound("Not find edificio".to_string()),
                _ => DomainError::Unexpected(e),
            })
    }
}

impl Insert<NewEdificio> for EdificioDAO {
    fn insert(conn: &mut PostgresPooled, item: NewEdificio) -> Result<Self::Item, DomainError> {
        diesel::insert_into(edificio::table)
            .values(&item)
            .get_result(conn)
            .map_err(|e| match e {
                Error::NotFound => DomainError::EdificioNotFound("Not find edificio".to_string()),
                Error::DatabaseError(kind, _) => {
                    if matches!(kind, diesel::result::DatabaseErrorKind::UniqueViolation) {
                        DomainError::EdificioAlreadyExists(format!(
                            "Edificio already exists: {}",
                            item.chiave
                        ))
                    } else {
                        DomainError::Unexpected(e)
                    }
                }
                _ => DomainError::Unexpected(e),
            })
    }
}

impl Update<UpdateEdificio, String> for EdificioDAO {
    fn update(
        conn: &mut PostgresPooled,
        id: String,
        item: UpdateEdificio,
    ) -> Result<Self::Item, DomainError> {
        diesel::update(edificio::table.find(id))
            .set(&item)
            .get_result(conn)
            .map_err(|e| match e {
                Error::NotFound => DomainError::EdificioNotFound("Not find edificio".to_string()),
                _ => DomainError::Unexpected(e),
            })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use diesel::r2d2::{ConnectionManager, Pool};
    use diesel::PgConnection;
    use std::sync::OnceLock;
    use std::time::Duration;

    static POOL_DB: OnceLock<Pool<ConnectionManager<PgConnection>>> = OnceLock::new();

    async fn create_postgres_pool() -> &'static Pool<ConnectionManager<PgConnection>> {
        POOL_DB.get_or_init(|| {
            let connection_string =
                "postgres://app_user:app_password@127.0.0.1:5432/app_development".to_string();
            let manager = ConnectionManager::<PgConnection>::new(connection_string);
            Pool::builder()
                .max_size(1)
                .connection_timeout(Duration::from_secs(1))
                .build(manager)
                .expect("Failed to create pool")
        })
    }

    #[tokio::test]
    async fn test_get_all() {
        let pool = create_postgres_pool().await;
        let mut conn = pool.get().unwrap();

        match EdificioDAO::get_all(&mut conn) {
            Ok(res) => {
                println!("{res:?}");
                println!("{}", res.len())
            }
            Err(e) => {
                panic!("{e:?}");
            }
        }
    }

    #[tokio::test]
    async fn test_insert_and_update() {
        let pool = create_postgres_pool().await;
        let mut conn = pool.get().unwrap();

        let insert_edificio = NewEdificio {
            chiave: "9999-999".to_string(),
            fascicolo: 125898,
            indirizzo: "Via test, 12345".to_string(),
        };

        let inserted_edificio = match EdificioDAO::insert(&mut conn, insert_edificio) {
            Ok(res) => {
                println!("{res:?}");
                assert_eq!(res.chiave, "9999-999");
                res
            }
            Err(e) => {
                panic!("{e:?}");
            }
        };

        let update_edificio = UpdateEdificio {
            anno_costruzione: None,
            anno_riqualificazione: Some(2021),
            note_riqualificazione: None,
            isolamento_tetto: None,
            cappotto: None,
        };

        match EdificioDAO::update(&mut conn, inserted_edificio.chiave.clone(), update_edificio) {
            Ok(res) => {
                println!("{res:?}");
                assert_eq!(res.anno_riqualificazione, Some(2021));
            }
            Err(e) => {
                panic!("{e:?}");
            }
        }

        diesel::sql_query("DELETE FROM edificio WHERE chiave = $1")
            .bind::<diesel::sql_types::Text, _>(&inserted_edificio.chiave)
            .execute(&mut conn)
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "EdificioNotFound")]
    async fn test_not_found() {
        let pool = create_postgres_pool().await;
        let mut conn = pool.get().unwrap();

        let update_edificio = UpdateEdificio {
            anno_costruzione: None,
            anno_riqualificazione: Some(2021),
            note_riqualificazione: None,
            isolamento_tetto: None,
            cappotto: None,
        };

        match EdificioDAO::update(&mut conn, "9999-99".to_string(), update_edificio) {
            Ok(res) => {
                println!("{res:?}");
                assert_eq!(res.anno_riqualificazione, Some(2021));
            }
            Err(e) => {
                panic!("{e:?}");
            }
        }
    }

    #[tokio::test]
    #[should_panic(expected = "EdificioAlreadyExists")]
    async fn test_insert_duplicate() {
        let pool = create_postgres_pool().await;
        let mut conn = pool.get().unwrap();

        let insert_edificio = NewEdificio {
            chiave: "9999-999".to_string(),
            fascicolo: 125898,
            indirizzo: "Via test, 12345".to_string(),
        };

        let _ = EdificioDAO::insert(&mut conn, insert_edificio.clone());
        if let Err(e) = EdificioDAO::insert(&mut conn, insert_edificio.clone()) {
            diesel::sql_query("DELETE FROM edificio WHERE chiave = $1")
                .bind::<diesel::sql_types::Text, _>(&insert_edificio.chiave)
                .execute(&mut conn)
                .unwrap();
            panic!("{e:?}");
        }
    }
}
