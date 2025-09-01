use crate::dao::{StanzaConInfissiDao, StanzaDAO};
use crate::dto::StanzaDTO;
use app_models::models::{StanzaConInfissi, UpdateStanzaConInfissi};
use app_state::selected_edificio::StateEdificioSelected;
pub use app_utils::{
    app_error::{AppResult, ApplicationError, DomainError},
    app_interface::{
        dao_interface::crud_operations::{Get, Insert, Update},
        database_interface::DatabaseManager,
        service_interface::{CreateService, UpdateService},
    },
};
use async_trait::async_trait;
use diesel::Connection;
use std::collections::HashMap;
use tauri::State;

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
                        std::iter::repeat_n(
                            infisso.infisso_id.clone(),
                            infisso.num_infisso as usize,
                        )
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
            let updated_stanza = StanzaDAO::update(tx, item.id as i32, item.clone().into())?;
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
                let stanza_con_infissi_update = UpdateStanzaConInfissi { num_infisso: count };
                let result = StanzaConInfissiDao::update(
                    tx,
                    (
                        updated_stanza.edificio_id.clone(),
                        updated_stanza.id,
                        infisso.clone(),
                    ),
                    stanza_con_infissi_update,
                )?;
                count_infissi
                    .entry(infisso)
                    .and_modify(|value| *value = result.num_infisso);
            }
            // Create the stanza dto and set the infissi field
            let mut stanza_dto = StanzaDTO::from(&updated_stanza);
            stanza_dto.infissi = Some(
                count_infissi
                    .iter()
                    .flat_map(|(id, count)| std::iter::repeat_n(id, *count as usize))
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>(),
            );

            Ok(stanza_dto)
        })
            .map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::InfissoDAO;
    use crate::dto::InfissoDTO;
    use crate::{
        dao::{EdificioDAO, StanzaDAO},
        dto::{EdificioDTO, StanzaDTO},
    };
    use app_state::{
        database::DatabaseManager,
        selected_edificio::{EdificioSelected, StateEdificioSelected},
    };
    use app_utils::test::utils::read_json_file;
    use app_utils::test::ResultTest;
    use app_utils::{
        app_interface::{
            dao_interface::crud_operations::Insert,
            database_interface::DatabaseManager as DatabaseManagerInterface,
        },
        path_data_fake,
        test::TestServiceEnvironment,
    };
    use tokio::sync::RwLock;

    const SELECTED_EDIFICIO_ID: &str = "6192-81";

    async fn setup_env_stanze() -> ResultTest<TestServiceEnvironment<DatabaseManager>> {
        let test_service_environment =
            TestServiceEnvironment::new::<_, _>(|db_manager: DatabaseManager| async move {
                let edifici_dto =
                    read_json_file::<EdificioDTO>(path_data_fake!("edificiFake").as_str())?;
                let stanze_dto =
                    read_json_file::<StanzaDTO>(path_data_fake!("stanzeFake").as_str())?;
                let infissi_dto =
                    read_json_file::<InfissoDTO>(path_data_fake!("infissiFake").as_str())?;
                {
                    let mut pool = db_manager.get_connection().await?;
                    for edificio_dto in edifici_dto {
                        let _ = EdificioDAO::insert(&mut pool, edificio_dto.into());
                    }
                    for stanza_dto in stanze_dto {
                        let _ = StanzaDAO::insert(&mut pool, stanza_dto.into());
                    }
                    for infisso_dto in infissi_dto {
                        let _ = InfissoDAO::insert(&mut pool, infisso_dto.into());
                    }
                }
                Ok(())
            })
                .await?;

        let select_edificio = StateEdificioSelected::new(RwLock::new(EdificioSelected::new()));
        select_edificio
            .write()
            .await
            .set_chiave(SELECTED_EDIFICIO_ID.to_string());

        test_service_environment.set_state_app(select_edificio);

        Ok(test_service_environment)
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_retrieve_stanze() -> ResultTest {
        let env = setup_env_stanze().await?;
        let state_db = env.database();
        let selected_edificio = env.state_app::<StateEdificioSelected>();

        match StanzaService::get_stanze_edificio(state_db, selected_edificio).await {
            Ok(result) => {
                assert_eq!(result.len(), 5);
            }
            Err(e) => panic!("{:?}", e),
        }

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_create_stanza() -> ResultTest {
        let env = setup_env_stanze().await?;
        let state_db = env.database();

        let stanza_dto = StanzaDTO {
            id: 0,
            edificio_id: SELECTED_EDIFICIO_ID.to_string(),
            piano: "5".to_string(),
            id_spazio: "78548".to_string(),
            cod_stanza: "PT5994".to_string(),
            destinazione_uso: "Bagno".to_string(),
            altezza: None,
            spessore_muro: None,
            riscaldamento: None,
            raffrescamento: None,
            illuminazione: None,
            infissi: None,
        };

        match StanzaService::create(state_db, stanza_dto).await {
            Ok(result) => {
                assert_eq!(result.id, 51);
                assert_eq!(result.piano.trim(), "5");
            }
            Err(e) => panic!("{:?}", e),
        }

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_update_stanza() -> ResultTest {
        let env = setup_env_stanze().await?;
        let state_db = env.database();

        let stanza_dto = StanzaDTO {
            id: 50,
            edificio_id: "6192-81".to_string(),
            piano: "1".to_string(),
            id_spazio: "SP050".to_string(),
            cod_stanza: "ST050".to_string(),
            destinazione_uso: "Bagno d'epoca".to_string(),
            altezza: Some(25u16),
            spessore_muro: None,
            riscaldamento: Some("Ventilconvettori".to_string()),
            raffrescamento: None,
            illuminazione: None,
            infissi: None,
        };

        match StanzaService::update(state_db, stanza_dto).await {
            Ok(result) => {
                assert_eq!(result.id, 50);
                assert_eq!(result.altezza.unwrap(), 25);
                assert_eq!(result.riscaldamento.unwrap(), "Ventilconvettori");
            }
            Err(e) => panic!("{:?}", e),
        }

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_update_stanza_with_infissi() -> ResultTest {
        let env = setup_env_stanze().await?;
        let state_db = env.database();

        let stanza_dto = StanzaDTO {
            id: 50,
            edificio_id: "6192-81".to_string(),
            piano: "1".to_string(),
            id_spazio: "SP050".to_string(),
            cod_stanza: "ST050".to_string(),
            destinazione_uso: "Bagno d'epoca".to_string(),
            altezza: Some(25u16),
            spessore_muro: None,
            riscaldamento: Some("Ventilconvettori".to_string()),
            raffrescamento: None,
            illuminazione: None,
            infissi: Some(vec![
                "A".to_string(),
                "A".to_string(),
                "B".to_string(),
                "B".to_string(),
            ]),
        };

        match StanzaService::update(state_db, stanza_dto).await {
            Ok(result) => {
                assert_eq!(result.id, 50);
                assert_eq!(
                    result.infissi,
                    Some(vec![
                        "A".to_string(),
                        "A".to_string(),
                        "B".to_string(),
                        "B".to_string()
                    ])
                );
                println!("{:#?}", result);
            }
            Err(e) => panic!("{:?}", e),
        }

        Ok(())
    }
}
