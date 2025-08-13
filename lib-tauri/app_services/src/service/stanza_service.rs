use crate::dao::{StanzaConInfissiDao, StanzaDAO};
use crate::dto::StanzaDTO;
use crate::service::{EdificioSelected, StateEdificioSelected};
use app_error::{AppResult, ApplicationError, DomainError};
use app_interface::dao_interface::crud_operations::{Get, Insert, Update};
use app_interface::database_interface::DatabaseManager;
use app_interface::service_interface::{CreateService, RetrieveManyService, UpdateService};
use app_models::models::{StanzaConInfissi, UpdateStanzaConInfissi};
use async_trait::async_trait;
use diesel::Connection;
use std::collections::HashMap;
use std::sync::Arc;
use tauri::test::MockRuntime;
use tauri::{App, State};

pub struct StanzaService;

impl StanzaService {
    pub async fn get_stanze_edificio(
        db: State<'_, impl DatabaseManager + Send + Sync>,
        stato_edificio: State<'_, StateEdificioSelected>,
    ) -> AppResult<Vec<StanzaDTO>> {
        let mut conn = db.get_connection().await?;
        let edificio_id = match stato_edificio.read().await.get_chiave() {
            Some(edificio_id) => edificio_id,
            None => return Err(ApplicationError::EdificioNotSelected),
        };

        conn.transaction::<_, DomainError, _>(|conn| {
            let stanze = StanzaDAO::get(conn, &edificio_id)?;
            let mut stanze_dto: Vec<StanzaDTO> = stanze.iter().map(StanzaDTO::from).collect();

            let infissi = StanzaConInfissiDao::get(conn, edificio_id.as_str())?;

            for stanza_dto in &mut stanze_dto {
                // Retrieve only the infissi of the current stanza
                let infissi: Vec<&StanzaConInfissi> = infissi
                    .iter()
                    .filter(|x| {
                        x.stanza_id == (stanza_dto.id as i32)
                            && x.edificio_id == edificio_id.as_str()
                    })
                    .collect();

                if infissi.is_empty() {
                    continue;
                }

                // Retrieve the infissi id of the current stanza and add them to the stanza dto
                let infissi_id = infissi
                    .iter()
                    .flat_map(|infisso| {
                        std::iter::repeat_n(infisso.infisso_id.clone(), infisso.num_infisso as usize)
                    })
                    .collect();

                stanza_dto.infissi = Some(infissi_id);
            }

            Ok(stanze_dto)
        })
        .map_err(|e| e.into())
    }
}

#[async_trait]
impl RetrieveManyService<StanzaDTO> for StanzaService {
    async fn retrieve_many(
        db: State<'_, impl DatabaseManager + Send + Sync>,
    ) -> AppResult<Vec<StanzaDTO>> {
        unreachable!("Questo codice non deve essere eseguito")
        // let mut conn = db.get_connection().await?;
        //
        // conn.transaction::<_, _, _>(|conn| {
        //     let stanze = StanzaDAO::get(conn)?;
        //     let mut stanze_dto: Vec<StanzaDTO> = stanze.iter().map(StanzaDTO::from).collect();
        //
        //     let infissi = StanzaConInfissiDao::get(conn, ())?;
        //
        //     for stanza_dto in &mut stanze_dto {
        //         let infissi = infissi
        //             .iter()
        //             .find(|x| x.stanza_id == (stanza_dto.id as i32));
        //         if let Some(infissi) = infissi {
        //             let infissi: Vec<String> = infissi.expanse_infissi();
        //             stanza.infissi = Some(infissi);
        //         } else {
        //             continue;
        //         }
        //     }
        //
        //     Ok(stanze_dto)
        // })
    }
}

#[async_trait]
impl CreateService<StanzaDTO> for StanzaService {
    async fn create(
        db: State<'_, impl DatabaseManager + Send + Sync>,
        item: StanzaDTO,
    ) -> AppResult<StanzaDTO> {
        let mut conn = db.get_connection().await?;
        let result = StanzaDAO::insert(&mut conn, item.into())?;
        Ok(StanzaDTO::from(&result))
    }
}

#[async_trait]
impl UpdateService<StanzaDTO> for StanzaService {
    async fn update(
        db: State<'_, impl DatabaseManager + Send + Sync>,
        item: StanzaDTO,
    ) -> AppResult<StanzaDTO> {
        let mut conn = db.get_connection().await?;

        conn.transaction::<_, DomainError, _>(|tx| {
            let updated_stanza =
                StanzaDAO::update(tx, item.id as i32, item.clone().into())?;
            if item.infissi.is_none() {
                return Ok(StanzaDTO::from(&updated_stanza));
            }

            // Collect the infissi and count them by infisso
            let mut count_infissi = HashMap::new();
            for infisso in item.infissi.unwrap() {
                *count_infissi.entry(infisso).or_insert(0) += 1;
            }
            // Update the infissi count in the database
            for (infisso, count) in count_infissi.clone() {
                let stanza_con_infissi_update = UpdateStanzaConInfissi {
                    num_infisso: count,
                };
                let result = StanzaConInfissiDao::update(
                    tx,
                    (
                        updated_stanza.edificio_id.clone(),
                        updated_stanza.id,
                        infisso.clone(),
                    ),
                    stanza_con_infissi_update,
                )?;
                count_infissi.entry(infisso).and_modify(|value| *value = result.num_infisso);
            }
            // Create the stanza dto and set the infissi field
            let mut stanza_dto = StanzaDTO::from(&updated_stanza);
            stanza_dto.infissi = Some(count_infissi
                .iter()
                .flat_map(|(id, count)| std::iter::repeat_n(id, *count as usize))
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
            );

            Ok(stanza_dto)
        })
        .map_err(|e| e.into())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::EdificioSelected;
    use app_error::database_error::DbError;
    use app_interface::database_interface::{DatabaseConnector, PostgresPooled};
    use app_utils::test::{get_connection_string, MockDatabaseConnector};
    use std::sync::Arc;
    use tauri::test::MockRuntime;
    use tauri::{App, Manager};
    use tokio::sync::RwLock;

    // Mock del DatabaseManager per i test
    #[derive(Clone)]
    struct MockDatabaseManager {
        connector: MockDatabaseConnector,
    }

    impl MockDatabaseManager {
        pub fn new(should_fail: bool) -> Self {
            Self {
                connector: MockDatabaseConnector {
                    postgres_should_fail: should_fail,
                },
            }
        }
    }

    // Implement DatabaseManager trait per il mock
    #[async_trait]
    impl DatabaseManager for MockDatabaseManager {
        async fn get_connection(&self) -> Result<PostgresPooled, DbError> {
            let pool = self.connector.create_postgres_pool().await;
            Ok(pool.get().unwrap())
        }
    }


    fn create_test_app() -> App<MockRuntime>{
        tauri::test::mock_app()
    }

    // Setup dell'app con gli stati necessari
    fn setup_app_with_states(app: &App<MockRuntime>) {
        // Setup database mock
        let mock_db = MockDatabaseManager::new(false);
        app.manage(mock_db);

        // Setup edificio state
        let edificio_state = Arc::new(RwLock::new(EdificioSelected::new()));
        app.manage(edificio_state);
    }




    #[tokio::test]
    async fn test_retrieve_data(){
        let app = create_test_app();
        setup_app_with_states(&app);

        let edificio_state: State<StateEdificioSelected> = app.state();
        {
            let mut edificio = edificio_state.write().await;
            edificio.set_chiave("test_edificio_123".to_string());
        }

        // Ottieni gli State
        {
            use std::process::Command;

            let url_db = get_connection_string().await;

            let output = Command::new("cargo")
                .args(["run", "--manifest-path", "../../migration_data/Cargo.toml", "--", url_db.as_str()])
                .output()
                .expect("Errore nell'esecuzione di migration_data");

            // Verifica che l'esecuzione sia andata a buon fine
            if !output.status.success() {
                panic!("migration_data ha fallito: {}", String::from_utf8_lossy(&output.stderr));
            }

            println!("migration_data eseguito con successo: {}", String::from_utf8_lossy(&output.stdout));
        }
        let db_state: State<MockDatabaseManager> = app.state();
        let edificio_state: State<StateEdificioSelected> = app.state();

        // Act
        let result = StanzaService::get_stanze_edificio(db_state, edificio_state).await;

        // Assert
        match result {
            Ok(stanze) => {
                println!("Stanze recuperate: {:#?}", stanze);
            }
            Err(e) => {
                panic!("Errore durante il test: {:?}", e);
            }
        }

    }
}