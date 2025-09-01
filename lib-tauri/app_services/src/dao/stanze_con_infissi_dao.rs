use app_models::{
    models::{StanzaConInfissi, UpdateStanzaConInfissi},
    schema::stanza_con_infissi,
};
use app_utils::{
    app_error::DomainError,
    app_interface::{
        dao_interface::{
            crud_operations::{Get, Insert, Update},
            DAO,
        },
        database_interface::PostgresPooled,
    },
};
use diesel::{result::Error, ExpressionMethods, QueryDsl, RunQueryDsl};

pub struct StanzaConInfissiDao;

impl DAO for StanzaConInfissiDao {}

impl<'a> Get<StanzaConInfissi, &'a str> for StanzaConInfissiDao {
    type Output = Vec<StanzaConInfissi>;

    /// Recupera tutti gli infissi di un edificio
    fn get(conn: &mut PostgresPooled, id: &'a str) -> Result<Self::Output, DomainError> {
        stanza_con_infissi::table
            .filter(stanza_con_infissi::edificio_id.eq(id))
            .get_results(conn)
            .map_err(|e| match e {
                Error::NotFound => DomainError::StanzaConInfissiNotFound,
                _ => DomainError::Unexpected(e),
            })
    }
}

impl Get<StanzaConInfissi, (String, i32)> for StanzaConInfissiDao {
    type Output = Vec<StanzaConInfissi>;
    /// L'id è una tuple di id che corrispondono -> (edificio, stanza)
    /// Recupera tutti gli infissi che sono collegati a una stanza
    fn get(conn: &mut PostgresPooled, id: (String, i32)) -> Result<Self::Output, DomainError> {
        stanza_con_infissi::table
            .filter(stanza_con_infissi::edificio_id.eq(id.0))
            .filter(stanza_con_infissi::stanza_id.eq(id.1))
            .get_results(conn)
            .map_err(|e| match e {
                Error::NotFound => DomainError::StanzaConInfissiNotFound,
                _ => DomainError::Unexpected(e),
            })
    }
}

impl Insert<StanzaConInfissi> for StanzaConInfissiDao {
    type Output = StanzaConInfissi;
    fn insert(
        conn: &mut PostgresPooled,
        item: StanzaConInfissi,
    ) -> Result<Self::Output, DomainError> {
        diesel::insert_into(stanza_con_infissi::table)
            .values(&item)
            .get_result(conn)
            .map_err(DomainError::from)
    }
}

impl Update<UpdateStanzaConInfissi, (String, i32, String)> for StanzaConInfissiDao {
    type Output = StanzaConInfissi;
    /// id -> (edificio, stanza, infisso)
    fn update(
        conn: &mut PostgresPooled,
        id: (String, i32, String),
        item: UpdateStanzaConInfissi,
    ) -> Result<Self::Output, DomainError> {
        let update_result = diesel::update(
            stanza_con_infissi::table
                .filter(stanza_con_infissi::edificio_id.eq(&id.0))
                .filter(stanza_con_infissi::stanza_id.eq(id.1))
                .filter(stanza_con_infissi::infisso_id.eq(&id.2)),
        )
        .set(stanza_con_infissi::num_infisso.eq(stanza_con_infissi::num_infisso + item.num_infisso))
        .get_result(conn);
        //
        // .map_err(|e| match e {
        //     Error::NotFound => DomainError::StanzaConInfissiNotFound,
        //     _ => DomainError::Unexpected(e),
        // });
        //
        match update_result {
            Ok(stanza_con_infissi) => Ok(stanza_con_infissi),
            Err(Error::NotFound) => {
                let new_entry = StanzaConInfissi {
                    infisso_id: id.2,
                    edificio_id: id.0,
                    stanza_id: id.1,
                    num_infisso: item.num_infisso,
                };
                Self::insert(conn, new_entry)
            }
            Err(e) => Err(DomainError::Unexpected(e)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::dao::{EdificioDAO, InfissoDAO, StanzaDAO};
    use crate::dto::{EdificioDTO, InfissoDTO, StanzaDTO};
    use app_models::models::{NewEdificio, NewInfisso, NewStanza};
    use app_utils::test::{ResultTest, TestDaoEnvironment};
    use serde::Deserialize;

    const ID_EDIFICIO: &str = "4693-182";
    const ID_STANZA: i32 = 27;

    async fn setup_env() -> ResultTest<TestDaoEnvironment> {
        let env = TestDaoEnvironment::new().await?;

        env.insert_data::<EdificioDAO, EdificioDTO, NewEdificio>("edificiFake")?;
        env.insert_data::<StanzaDAO, StanzaDTO, NewStanza>("stanzeFake")?;
        env.insert_data::<InfissoDAO, InfissoDTO, NewInfisso>("infissiFake")?;

        #[derive(Deserialize)]
        struct StanzaConInfissiDTO {
            infisso_id: String,
            edificio_id: String,
            stanza_id: i32,
            num_infisso: i32,
        }

        impl From<StanzaConInfissiDTO> for StanzaConInfissi {
            fn from(value: StanzaConInfissiDTO) -> Self {
                Self {
                    infisso_id: value.infisso_id,
                    edificio_id: value.edificio_id,
                    stanza_id: value.stanza_id,
                    num_infisso: value.num_infisso,
                }
            }
        }

        env.insert_data::<StanzaConInfissiDao, StanzaConInfissiDTO, StanzaConInfissi>(
            "stanzeConInfissiFake",
        )?;

        Ok(env)
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_get_data() -> ResultTest {
        let env = setup_env().await?;
        let mut conn = env.get_pooled_connection()?;

        match StanzaConInfissiDao::get(&mut conn, (ID_EDIFICIO.to_string(), ID_STANZA)) {
            Ok(stanza_con_infissi) => {
                // It's the kind of finestre found and not the number of finestre in the stanza
                assert_eq!(stanza_con_infissi.len(), 3);
                println!("{stanza_con_infissi:#?}")
            }
            Err(e) => panic!("{e:?}"),
        }

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_insert_data() -> ResultTest {
        let env = setup_env().await?;
        let mut conn = env.get_pooled_connection()?;

        let insert_data = StanzaConInfissi {
            infisso_id: "C".to_string(),
            edificio_id: ID_EDIFICIO.to_string(),
            stanza_id: 26,
            num_infisso: 5,
        };

        match StanzaConInfissiDao::insert(&mut conn, insert_data.clone()) {
            Ok(mut stanza_con_infissi) => {
                stanza_con_infissi.infisso_id = stanza_con_infissi.infisso_id.trim().to_string();

                assert_eq!(stanza_con_infissi, insert_data);
                println!("Test insert data: success");
            }
            Err(e) => panic!("{e:?}"),
        }

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_update_data() -> ResultTest {
        let env = setup_env().await?;
        let mut conn = env.get_pooled_connection()?;

        let update_data = UpdateStanzaConInfissi { num_infisso: 10 };

        match StanzaConInfissiDao::update(
            &mut conn,
            (ID_EDIFICIO.to_string(), ID_STANZA, "C".to_string()),
            update_data,
        ) {
            Ok(mut stanza_con_infissi) => {
                stanza_con_infissi.infisso_id = stanza_con_infissi.infisso_id.trim().to_string();

                assert_eq!(stanza_con_infissi.num_infisso, 12);
                println!("Test update data: success");
            }
            Err(e) => panic!("{e:?}"),
        }

        Ok(())
    }
}
