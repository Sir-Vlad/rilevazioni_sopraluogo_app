use app_models::{
    models::{Edificio, NewEdificio, UpdateEdificio},
    schema::edificio,
};
use app_state::database::database_manager::PostgresPooled;
use app_utils::{
    app_error::DomainError,
    app_interface::dao_interface::{
        DAO,
        crud_operations::{Get, GetAll, Insert, Update},
    },
};
use diesel::{QueryDsl, RunQueryDsl};

use crate::dao::utils::{EntityType, map_error_for_entity};

pub struct EdificioDAO;

impl DAO for EdificioDAO {}

impl GetAll<Edificio> for EdificioDAO {
    type Output = Edificio;

    fn get_all(conn: &mut PostgresPooled) -> Result<Vec<Self::Output>, DomainError> {
        edificio::table
            .load::<Edificio>(conn)
            .map_err(|e| map_error_for_entity(e, EntityType::Edificio))
    }
}

impl Get<Edificio, String> for EdificioDAO {
    type Output = Edificio;

    fn get(conn: &mut PostgresPooled, id: String) -> Result<Self::Output, DomainError> {
        edificio::table
            .find(id)
            .first::<Edificio>(conn)
            .map_err(|e| map_error_for_entity(e, EntityType::Edificio))
    }
}

impl Insert<NewEdificio<'_>> for EdificioDAO {
    type Output = Edificio;

    fn insert(conn: &mut PostgresPooled, item: NewEdificio) -> Result<Self::Output, DomainError> {
        diesel::insert_into(edificio::table)
            .values(&item)
            .get_result(conn)
            .map_err(|e| map_error_for_entity(e, EntityType::Edificio))
    }
}

impl Update<UpdateEdificio<'_>, String> for EdificioDAO {
    type Output = Edificio;

    fn update(
        conn: &mut PostgresPooled,
        id: String,
        item: UpdateEdificio,
    ) -> Result<Self::Output, DomainError> {
        diesel::update(edificio::table.find(id))
            .set(&item)
            .get_result(conn)
            .map_err(|e| map_error_for_entity(e, EntityType::Edificio))
    }
}

#[cfg(test)]
mod test {
    use app_utils::test::create_postgres_pool;

    use super::*;

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
            chiave: "9999-999".into(),
            fascicolo: 125898,
            indirizzo: "Via test, 12345".into(),
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
            chiave: "9999-999".into(),
            fascicolo: 125898,
            indirizzo: "Via test, 12345".into(),
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
