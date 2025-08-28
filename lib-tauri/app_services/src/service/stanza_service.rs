use crate::dao::{StanzaConInfissiDao, StanzaDAO};
use crate::dto::StanzaDTO;
use app_error::{AppResult, ApplicationError, DomainError};
use app_interface::dao_interface::crud_operations::{Get, Insert, Update};
use app_interface::database_interface::DatabaseManager;
use app_interface::service_interface::{CreateService, UpdateService};
use app_models::models::{StanzaConInfissi, UpdateStanzaConInfissi};
use app_state::selected_edificio::StateEdificioSelected;
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
    use crate::dao::StanzaDAO;
    use crate::dto::StanzaDTO;
    use app_interface::dao_interface::crud_operations::Insert;
    use app_interface::database_interface::DatabaseManager as DatabaseManagerInterface;
    use app_state::database::DatabaseManager;
    use app_utils::test::{read_json_file, TestServiceEnvironment};
    use std::error::Error;

    const FILE_PATH_DATA_FAKE: &str = "../dataFake/stanzeFake.json";

    async fn setup_env_stanze() -> Result<TestServiceEnvironment<DatabaseManager>, Box<dyn Error>> {
        TestServiceEnvironment::new::<_, _>(|db_manager: DatabaseManager| async move {
            let data = read_json_file::<StanzaDTO>(FILE_PATH_DATA_FAKE)?;
            {
                let mut pool = db_manager.get_connection().await?;
                for stanza in data {
                    // Ignora errori di duplicati nei test
                    let _ = StanzaDAO::insert(&mut pool, stanza.into());
                }
            }
            Ok(())
        })
            .await
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_retrieve_stanze() -> Result<(), Box<dyn Error>> {
        let env = setup_env_stanze().await?;
        let state_db = env.database();


        // match StanzaService::get_stanze_edificio(state_db) {}


        Ok(())
    }
}
