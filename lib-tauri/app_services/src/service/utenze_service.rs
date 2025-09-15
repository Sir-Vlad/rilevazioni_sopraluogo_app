use crate::dao::UtenzeDAO;
use crate::dto::UtenzaDTO;
use app_state::selected_edificio::SelectedEdificioTrait;
use app_utils::app_error::{AppResult, ApplicationError};
use app_utils::app_interface::dao_interface::crud_operations::{Get, GetAll, Insert};
use app_utils::app_interface::database_interface::DatabaseManagerTrait;
use app_utils::app_interface::service_interface::{
    CreateService, RetrieveByEdificioSelected, RetrieveManyService, SelectedEdificioState,
};
use async_trait::async_trait;
use std::ops::Deref;
use tauri::State;

pub struct UtenzeService;

#[async_trait]
impl RetrieveManyService<UtenzaDTO> for UtenzeService {
    async fn retrieve_many(
        db: State<'_, impl DatabaseManagerTrait + Send + Sync>,
    ) -> Result<Vec<UtenzaDTO>, ApplicationError> {
        let mut conn = db.get_connection().await?;
        let utenze = UtenzeDAO::get_all(&mut conn)?;
        Ok(utenze.iter().map(UtenzaDTO::from).collect())
    }
}

#[async_trait]
impl RetrieveByEdificioSelected<UtenzaDTO> for UtenzeService {
    async fn retrieve_by_edificio_selected<S>(
        db_state: State<'_, impl DatabaseManagerTrait + Send + Sync>,
        edificio_selected_state: State<'_, SelectedEdificioState<S>>,
    ) -> AppResult<Vec<UtenzaDTO>>
    where
        S: SelectedEdificioTrait + Send + Sync,
    {
        let mut conn = db_state.get_connection().await?;
        let edificio_selected = edificio_selected_state.read().await.deref().get_chiave();
        if edificio_selected.is_none() {
            return Err(ApplicationError::EdificioNotSelected);
        }

        let utenze = UtenzeDAO::get(&mut conn, edificio_selected.unwrap())?;
        Ok(utenze.iter().map(UtenzaDTO::from).collect())
    }
}

#[async_trait]
impl CreateService<UtenzaDTO> for UtenzeService {
    async fn create(
        db: State<'_, impl DatabaseManagerTrait + Send + Sync>,
        utenza: UtenzaDTO,
    ) -> Result<UtenzaDTO, ApplicationError> {
        let mut conn = db.get_connection().await?;
        let utenza = UtenzeDAO::insert(&mut conn, utenza.into())?;
        Ok(UtenzaDTO::from(&utenza))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::EdificioDAO;
    use crate::dto::EdificioDTO;
    use app_models::models::TipoUtenza;
    use app_state::database::DatabaseManager;
    use app_state::selected_edificio::EdificioSelected;
    use app_utils::app_interface::database_interface::DatabaseManagerTrait;
    use app_utils::app_interface::service_interface::SelectedEdificioTrait;
    use app_utils::path_data_fake;
    use app_utils::test::utils::read_json_file;
    use app_utils::test::{ResultTest, TestServiceEnvironment};
    use std::ops::Deref;
    use tokio::io::AsyncReadExt;
    use tokio::sync::RwLock;

    async fn setup_utenze_env() -> ResultTest<TestServiceEnvironment<DatabaseManager>> {
        let test_service_env =
            TestServiceEnvironment::new::<_, _>(|db_manager: DatabaseManager| async move {
                let edifici_dto =
                    read_json_file::<EdificioDTO>(path_data_fake!("edificiFake").as_str())?;
                let utenze_dto =
                    read_json_file::<UtenzaDTO>(path_data_fake!("utenzeFake").as_str())?;

                {
                    let mut conn = db_manager.get_connection().await?;

                    for edificio_dto in edifici_dto {
                        let _ = EdificioDAO::insert(&mut conn, edificio_dto.into());
                    }
                    for utenza_dto in utenze_dto {
                        let _ = UtenzeDAO::insert(&mut conn, utenza_dto.into());
                    }
                }

                Ok(())
            })
                .await?;

        let select_edificio = SelectedEdificioState::new(RwLock::new(EdificioSelected::new()));
        select_edificio
            .write()
            .await
            .set_chiave("8361-122".to_string());
        test_service_env.set_state_app(select_edificio);

        Ok(test_service_env)
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_retrieve_utenze() -> ResultTest {
        let env = setup_utenze_env().await?;
        let state_db = env.database();

        match UtenzeService::retrieve_many(state_db).await {
            Ok(result) => {
                assert_eq!(result.len(), 3);
            }
            Err(e) => panic!("{:?}", e),
        }

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_create_utenze() -> ResultTest {
        let env = setup_utenze_env().await?;
        let state_db = env.database();
        let selected_edificio = env.state_app::<SelectedEdificioState<EdificioSelected>>();

        let insert_utenza = UtenzaDTO {
            id: 0,
            edificio_id: selected_edificio
                .inner()
                .deref()
                .read()
                .await
                .get_chiave()
                .unwrap(),
            tipo: TipoUtenza::Acqua,
            cod_contatore: "TEST-COD-ACQUA".to_string(),
            indirizzo_contatore: None,
        };

        match UtenzeService::create(state_db, insert_utenza).await {
            Ok(result) => {
                assert_eq!(result.id, 4);
                assert_eq!(result.tipo, TipoUtenza::Acqua);
            }
            Err(e) => panic!("{:?}", e),
        }

        Ok(())
    }
}
