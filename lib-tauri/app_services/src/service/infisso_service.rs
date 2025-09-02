use crate::dao::InfissoDAO;
use crate::dto::InfissoDTO;
use app_state::selected_edificio::StateEdificioSelected;
use app_utils::{
    app_error::AppResult,
    app_interface::{
        dao_interface::crud_operations::{Get, Insert, Update},
        database_interface::DatabaseManager,
        service_interface::{
            CreateService, UpdateService,
        }
    }
};
use async_trait::async_trait;
use tauri::State;

pub struct InfissoService;

impl InfissoService {
    pub async fn retrieve_infissi_by_edificio(
        db: State<'_, impl DatabaseManager + Send + Sync>,
        selected_edificio: State<'_, StateEdificioSelected>,
    ) -> AppResult<Vec<InfissoDTO>> {
        let mut conn = db.get_connection().await?;
        let edificio_id = match selected_edificio.read().await.get_chiave() {
            Some(chiave) => chiave,
            None => return Ok(Vec::new()),
        };
        let result = InfissoDAO::get(&mut conn, edificio_id)?;
        Ok(result.iter().map(InfissoDTO::from).collect())
    }
}

#[async_trait]
impl CreateService<InfissoDTO> for InfissoService {
    async fn create(
        db: State<'_, impl DatabaseManager + Send + Sync>,
        item: InfissoDTO,
    ) -> AppResult<InfissoDTO> {
        let mut conn = db.get_connection().await?;
        let result = InfissoDAO::insert(&mut conn, item.into())?;
        Ok(InfissoDTO::from(&result))
    }
}

#[async_trait]
impl UpdateService<InfissoDTO> for InfissoService {
    async fn update(
        db: State<'_, impl DatabaseManager + Send + Sync>,
        item: InfissoDTO,
    ) -> AppResult<InfissoDTO> {
        let mut conn = db.get_connection().await?;
        let result = InfissoDAO::update(
            &mut conn,
            (item.id.clone(), item.id_edificio.clone()),
            item.into(),
        )?;
        Ok(InfissoDTO::from(&result))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::dao::EdificioDAO;
    use crate::dto::EdificioDTO;
    use app_state::database::DatabaseManager;
    use app_state::selected_edificio::EdificioSelected;
    use app_utils::app_interface::database_interface::DatabaseManager as DatabaseManagerInterface;
    use app_utils::path_data_fake;
    use app_utils::test::utils::read_json_file;
    use app_utils::test::{ResultTest, TestServiceEnvironment};
    use std::ops::Deref;
    use tokio::sync::RwLock;

    async fn setup_env_infissi() -> ResultTest<TestServiceEnvironment<DatabaseManager>> {
        let test_service_env =
            TestServiceEnvironment::new::<_, _>(|db_manager: DatabaseManager| async move {
                let edifici_dto =
                    read_json_file::<EdificioDTO>(path_data_fake!("edificiFake").as_ref())?;
                let infissi_dto =
                    read_json_file::<InfissoDTO>(path_data_fake!("infissiFake").as_ref())?;
                {
                    let mut conn = db_manager.get_connection().await?;

                    for edificio_dto in edifici_dto {
                        let _ = EdificioDAO::insert(&mut conn, edificio_dto.into());
                    }
                    for infisso_dto in infissi_dto {
                        let _ = InfissoDAO::insert(&mut conn, infisso_dto.into());
                    }
                }

                Ok(())
            })
                .await?;

        let select_edificio = StateEdificioSelected::new(RwLock::new(EdificioSelected::new()));
        select_edificio
            .write()
            .await
            .set_chiave("8361-122".to_string());
        test_service_env.set_state_app(select_edificio);

        Ok(test_service_env)
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_retrieve_infissi() -> ResultTest {
        let env = setup_env_infissi().await?;
        let state_db = env.database();
        let selected_edificio = env.state_app::<StateEdificioSelected>();

        match InfissoService::retrieve_infissi_by_edificio(state_db, selected_edificio).await {
            Ok(result) => {
                assert_eq!(result.len(), 2)
            }
            Err(e) => panic!("{:?}", e),
        }
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_create_infissi() -> ResultTest {
        let env = setup_env_infissi().await?;
        let state_db = env.database();
        let selected_edificio = env.state_app::<StateEdificioSelected>();

        let insert_infisso = InfissoDTO {
            id: "C".to_string(),
            id_edificio: selected_edificio
                .inner()
                .deref()
                .read()
                .await
                .get_chiave()
                .unwrap(),
            tipo: "Porta".to_string(),
            altezza: 230,
            larghezza: 100,
            materiale: "Legno".to_string(),
            vetro: "Singolo".to_string(),
        };

        match InfissoService::create(state_db, insert_infisso.clone()).await {
            Ok(result) => {
                assert_eq!(result, insert_infisso);
            }
            Err(e) => panic!("{:?}", e),
        }
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_update_infissi() -> ResultTest {
        let env = setup_env_infissi().await?;
        let state_db = env.database();
        let selected_edificio = env.state_app::<StateEdificioSelected>();

        let insert_infisso = InfissoDTO {
            id: "B".to_string(),
            id_edificio: selected_edificio
                .inner()
                .deref()
                .read()
                .await
                .get_chiave()
                .unwrap(),
            tipo: "Finestra".to_string(),
            altezza: 230,
            larghezza: 100,
            materiale: "Legno".to_string(),
            vetro: "Singolo".to_string(),
        };

        match InfissoService::update(state_db, insert_infisso.clone()).await {
            Ok(result) => {
                assert_eq!(result, insert_infisso);
            }
            Err(e) => panic!("{:?}", e),
        }
        Ok(())
    }
}
