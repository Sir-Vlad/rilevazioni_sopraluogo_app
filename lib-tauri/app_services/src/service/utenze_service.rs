use std::ops::Deref;

use app_state::selected_edificio::SelectedEdificioTrait;
use app_utils::{
    app_error::{AppResult, ApplicationError, ErrorKind},
    app_interface::{
        dao_interface::crud_operations::{Get, GetAll, Insert},
        database_interface::DatabaseManagerTrait,
        service_interface::{
            CreateService, RetrieveBy, RetrieveByEdificioSelected, RetrieveManyService,
            SelectedEdificioState,
        },
    },
};
use async_trait::async_trait;
use tauri::State;

use crate::{dao::UtenzeDAO, dto::UtenzaDTO, service::DomainError};

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
impl RetrieveBy<UtenzaDTO> for UtenzeService {
    type Output = Vec<UtenzaDTO>;

    async fn retrieve_by(
        db_state: State<'_, impl DatabaseManagerTrait + Send + Sync>,
        where_field: &str,
        where_value: &str,
    ) -> AppResult<Self::Output> {
        let mut conn = db_state.get_connection().await?;

        let result = match where_field {
            "edificio" => UtenzeDAO::get(&mut conn, where_value.to_string())?,
            _ => {
                return Err(DomainError::InvalidInput(
                    ErrorKind::InvalidField,
                    where_field.to_string(),
                )
                .into());
            }
        };

        Ok(result.iter().map(UtenzaDTO::from).collect())
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
        let edificio_selected = edificio_selected_state.read().await.deref().get_chiave();
        if edificio_selected.is_none() {
            return Err(ApplicationError::EdificioNotSelected);
        }

        Self::retrieve_by(db_state, "edificio", edificio_selected.unwrap().deref()).await
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
    use std::ops::Deref;

    use app_models::models::TipoUtenza;
    use app_state::{database::DatabaseManager, selected_edificio::EdificioSelected};
    use app_utils::{
        app_interface::{
            database_interface::DatabaseManagerTrait, service_interface::SelectedEdificioTrait,
        },
        path_data_fake,
        test::{ResultTest, TestServiceEnvironment, utils::read_json_file},
    };
    use tokio::sync::RwLock;

    use super::*;
    use crate::{dao::EdificioDAO, dto::EdificioDTO};

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
