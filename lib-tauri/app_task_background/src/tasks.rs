use crate::{BackgroundTask, ResultTask};
use app_data_processing::IdGeneratorStanza;
use app_models::models::Stanza;
use app_services::service::{DomainError, EdificioService, StanzaService};
use app_state::database::DatabaseManager;
use app_utils::app_error::{ApplicationError, ErrorTask, TauriError};
use app_utils::app_interface::{
    database_interface::DatabaseManagerTrait,
    service_interface::{RetrieveBy, RetrieveManyService},
};
use async_trait::async_trait;
use diesel::{
    sql_types::{Integer, Text},
    RunQueryDsl,
};
use log::{debug, info};
use std::{collections::HashSet, sync::Arc};
use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_store::StoreExt;

pub struct IdStanzeProcessor<R: Runtime> {
    app_handle: Arc<AppHandle<R>>,
}

impl<R: Runtime> IdStanzeProcessor<R> {
    const FILE_SAVE_EDIFICI: &'static str = "migration_id_stanze.json";
    const KEY_EDIFICI: &'static str = "edifici_processed";

    pub fn new(app_handle: Arc<AppHandle<R>>) -> Self {
        Self { app_handle }
    }
}

#[async_trait]
impl<R: Runtime> BackgroundTask for IdStanzeProcessor<R> {
    const INTERVAL_SEC: u64 = 60; // 1 min
    const TASK_NAME: &'static str = "IdStanzeProcessor";

    async fn run(&mut self) -> ResultTask {
        /*
        Ogni volta che viene eseguita si sceglie un nuovo id edificio da aggiornare
         */
        info!("Starting task processor id stanze");

        let db_state = self.app_handle.state::<DatabaseManager>();
        let edifici = EdificioService::retrieve_many(db_state.clone()).await?;

        // Find the first element that has not been changed
        let store = self
            .app_handle
            .store(Self::FILE_SAVE_EDIFICI)
            .map_err(|e| ApplicationError::Tauri(TauriError::Plugin(e.into())))?;

        let edifici_stored = store.get(Self::KEY_EDIFICI);
        let mut edificio_processing: Option<String> = None;

        match edifici_stored {
            Some(value) => match value.as_array() {
                Some(edifici_array) => {
                    let mut edifici_saved = edifici_array
                        .iter()
                        .map(|v| {
                            let res = match v.as_str() {
                                Some(n) => n.to_string(),
                                None => {
                                    store.clear();
                                    return Err(ApplicationError::BackgroundTask(
                                        ErrorTask::Generic(
                                            "The value in the stored value is not a string"
                                                .to_string(),
                                        ),
                                    ));
                                }
                            };
                            Ok(res)
                        })
                        .collect::<ResultTask<Vec<String>>>()?;

                    let edifici_saved_set: HashSet<String> =
                        HashSet::from_iter(edifici_saved.clone());
                    let edifici_set: HashSet<String> = HashSet::from_iter(
                        edifici
                            .iter()
                            .map(|v| v.chiave.clone())
                            .collect::<Vec<String>>(),
                    );

                    let difference = edifici_set
                        .difference(&edifici_saved_set)
                        .collect::<Vec<_>>();

                    if difference.is_empty() {
                        info!("All edifici are already processed, nothing to do");
                        return Ok(());
                    }

                    edificio_processing = Some(difference[0].clone());

                    println!("edifici saved: {:?}", edificio_processing);
                    edifici_saved.push(edificio_processing.clone().unwrap());
                    store.set(Self::KEY_EDIFICI, edifici_saved);
                }
                None => {
                    store.clear();
                    return Err(ApplicationError::BackgroundTask(ErrorTask::Generic(
                        "The stored value is not an array".to_string(),
                    )));
                }
            },
            None => {
                edificio_processing = Some(edifici[0].chiave.clone());
                store.set(Self::KEY_EDIFICI, vec![edificio_processing.clone().unwrap()]);
            }
        }

        if let Some(edificio) = edificio_processing {
            info!("Processing edificio with id {}", edificio);

            let stanze = StanzaService::retrieve_by(
                db_state.clone(),
                "edificio",
                edificio.to_string().as_str(),
            )
                .await?;

            let mut id_generator_stanza = IdGeneratorStanza::new();
            let mut conn = db_state.get_connection().await?;

            stanze.into_iter().try_for_each::<_, ResultTask>(|stanza| {
                debug!("Before: {}", stanza.cod_stanza);
                let new_stanza = id_generator_stanza.generate_id(stanza.into())?;

                let stanza_updated: Stanza = diesel::sql_query(
                    "UPDATE stanza SET cod_stanza = $1 WHERE id = $2 RETURNING *;",
                )
                    .bind::<Text, _>(new_stanza.cod_stanza)
                    .bind::<Integer, _>(new_stanza.id)
                    .get_result(&mut conn)
                    .map_err(|e| ApplicationError::Domain(DomainError::from(e)))?;

                debug!("After: {:?}", stanza_updated.cod_stanza);

                Ok(())
            })?;
        }

        store
            .save()
            .map_err(|e| ApplicationError::Tauri(TauriError::Plugin(e.into())))?;
        store.close_resource();

        info!("Finish processing");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use app_models::models::NewStanza;
    use app_services::dao::{EdificioDAO, InfissoDAO, StanzaDAO};
    use app_services::dto::{EdificioDTO, InfissoDTO, StanzaDTO};
    use app_services::service::DatabaseManagerTrait;
    use app_utils::app_interface::dao_interface::crud_operations::Insert;
    use app_utils::path_data_fake;
    use app_utils::test::utils::read_json_file;
    use app_utils::test::{ResultTest, TestServiceEnvironment};
    use std::sync::Once;
    use tauri::test::MockRuntime;

    static LOGGER: Once = Once::new();

    struct TestEnv<D> {
        test_service_environment: TestServiceEnvironment<D>,
        id_stanze_processor: IdStanzeProcessor<MockRuntime>,
    }

    impl<D> TestEnv<D>
    where
        D: DatabaseManagerTrait + Send + Sync + 'static,
    {
        pub async fn new<T, F>(insert_data: T) -> Self
        where
            T: Fn(D) -> F + Send + 'static,
            F: std::future::Future<Output=ResultTest<()>> + Send + 'static,
            D: Clone,
        {
            let env = TestServiceEnvironment::new(insert_data)
                .await
                .expect("Could not connect to Test instance");

            let app = env.app();
            app.plugin(tauri_plugin_store::Builder::new().build())
                .expect("Could not build app plugin");

            let processor = IdStanzeProcessor::new(app);

            Self {
                test_service_environment: env,
                id_stanze_processor: processor,
            }
        }
    }

    async fn setup() -> TestEnv<DatabaseManager> {
        TestEnv::new::<_, _>(|db_manager: DatabaseManager| async move {
            let edifici_dto =
                read_json_file::<EdificioDTO>(path_data_fake!("edificiFake").as_str())?;
            let stanze_dto = read_json_file::<StanzaDTO>(path_data_fake!("stanzeFake").as_str())?;
            let infissi_dto =
                read_json_file::<InfissoDTO>(path_data_fake!("infissiFake").as_str())?;
            {
                let mut pool = db_manager.get_connection().await?;
                for edificio_dto in edifici_dto {
                    let _ = EdificioDAO::insert(&mut pool, edificio_dto.into());
                }
                for stanza_dto in stanze_dto {
                    let new_stanza: NewStanza = stanza_dto.into();
                    let _ = StanzaDAO::insert(&mut pool, new_stanza);
                }
                for infisso_dto in infissi_dto {
                    let _ = InfissoDAO::insert(&mut pool, infisso_dto.into());
                }
            }

            Ok(())
        })
            .await
    }

    fn init_logger() {
        LOGGER.call_once(|| {
            env_logger::builder()
                .is_test(true)
                .try_init().ok();
        });
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_run() -> ResultTest {
        init_logger();

        let mut test_env = setup().await;
        let store = test_env
            .test_service_environment
            .app()
            .store(IdStanzeProcessor::<MockRuntime>::FILE_SAVE_EDIFICI)?;
        store.delete(IdStanzeProcessor::<MockRuntime>::KEY_EDIFICI);


        test_env.id_stanze_processor.run().await?;

        Ok(())
    }
}
