use app_utils::{
    app_error::AppResult,
    app_interface::{
        dao_interface::crud_operations::{GetAll, Insert},
        database_interface::DatabaseManagerTrait,
        service_interface::{CreateService, RetrieveManyService},
    },
};
use async_trait::async_trait;
use tauri::State;

use crate::{
    dao::{AnnotazioneEdificioDAO, AnnotazioneInfissoDAO, AnnotazioneStanzaDAO},
    dto::{AnnotazioneEdificioDTO, AnnotazioneInfissoDTO, AnnotazioneStanzaDTO},
};

pub struct AnnotazioneService;

#[async_trait]
impl RetrieveManyService<AnnotazioneEdificioDTO> for AnnotazioneService {
    async fn retrieve_many(
        db: State<'_, impl DatabaseManagerTrait + Send + Sync>,
    ) -> AppResult<Vec<AnnotazioneEdificioDTO>> {
        let mut conn = db.get_connection().await?;
        let result = AnnotazioneEdificioDAO::get_all(&mut conn)?;
        Ok(result
            .into_iter()
            .map(AnnotazioneEdificioDTO::from)
            .collect())
    }
}

#[async_trait]
impl CreateService<AnnotazioneEdificioDTO> for AnnotazioneService {
    async fn create(
        db: State<'_, impl DatabaseManagerTrait + Send + Sync>,
        item: AnnotazioneEdificioDTO,
    ) -> AppResult<AnnotazioneEdificioDTO> {
        let mut conn = db.get_connection().await?;
        let result = AnnotazioneEdificioDAO::insert(&mut conn, item.into())?;
        Ok(AnnotazioneEdificioDTO::from(result))
    }
}

#[async_trait]
impl RetrieveManyService<AnnotazioneStanzaDTO> for AnnotazioneService {
    async fn retrieve_many(
        db: State<'_, impl DatabaseManagerTrait + Send + Sync>,
    ) -> AppResult<Vec<AnnotazioneStanzaDTO>> {
        let mut conn = db.get_connection().await?;
        let result = AnnotazioneStanzaDAO::get_all(&mut conn)?;
        Ok(result.into_iter().map(AnnotazioneStanzaDTO::from).collect())
    }
}

#[async_trait]
impl CreateService<AnnotazioneStanzaDTO> for AnnotazioneService {
    async fn create(
        db: State<'_, impl DatabaseManagerTrait + Send + Sync>,
        item: AnnotazioneStanzaDTO,
    ) -> AppResult<AnnotazioneStanzaDTO> {
        let mut conn = db.get_connection().await?;
        let result = AnnotazioneStanzaDAO::insert(&mut conn, item.into())?;
        Ok(AnnotazioneStanzaDTO::from(result))
    }
}

#[async_trait]
impl RetrieveManyService<AnnotazioneInfissoDTO> for AnnotazioneService {
    async fn retrieve_many(
        db: State<'_, impl DatabaseManagerTrait + Send + Sync>,
    ) -> AppResult<Vec<AnnotazioneInfissoDTO>> {
        let mut conn = db.get_connection().await?;
        let result = AnnotazioneInfissoDAO::get_all(&mut conn)?;
        Ok(result
            .into_iter()
            .map(AnnotazioneInfissoDTO::from)
            .collect())
    }
}

#[async_trait]
impl CreateService<AnnotazioneInfissoDTO> for AnnotazioneService {
    async fn create(
        db: State<'_, impl DatabaseManagerTrait + Send + Sync>,
        item: AnnotazioneInfissoDTO,
    ) -> AppResult<AnnotazioneInfissoDTO> {
        let mut conn = db.get_connection().await?;
        let result = AnnotazioneInfissoDAO::insert(&mut conn, item.into())?;
        Ok(AnnotazioneInfissoDTO::from(result))
    }
}

#[cfg(test)]
mod tests {
    use app_models::models::NewStanza;
    use app_state::database::DatabaseManager;
    use app_utils::{
        app_interface::{
            dao_interface::crud_operations::Insert,
            database_interface::DatabaseManagerTrait,
            service_interface::{CreateService, RetrieveManyService},
        },
        path_data_fake,
        test::{ResultTest, TestServiceEnvironment, utils::read_json_file},
    };

    use crate::{
        dao::{
            AnnotazioneEdificioDAO, AnnotazioneInfissoDAO, AnnotazioneStanzaDAO, EdificioDAO,
            InfissoDAO, StanzaDAO,
        },
        dto::{
            AnnotazioneEdificioDTO, AnnotazioneInfissoDTO, AnnotazioneStanzaDTO, EdificioDTO,
            InfissoDTO, StanzaDTO,
        },
        service::AnnotazioneService,
    };

    async fn setup_env_annotazione() -> ResultTest<TestServiceEnvironment<DatabaseManager>> {
        TestServiceEnvironment::new::<_, _>(|db_manager: DatabaseManager| async move {
            let edifici_dto =
                read_json_file::<EdificioDTO>(path_data_fake!("edificiFake").as_str())?;
            let stanze_dto = read_json_file::<StanzaDTO>(path_data_fake!("stanzeFake").as_str())?;
            let infissi_dto =
                read_json_file::<InfissoDTO>(path_data_fake!("infissiFake").as_str())?;

            let ann_edifici_dto = read_json_file::<AnnotazioneEdificioDTO>(
                path_data_fake!("annotazioniEdificioFake").as_str(),
            )?;
            let ann_stanze_dto = read_json_file::<AnnotazioneStanzaDTO>(
                path_data_fake!("annotazioniStanzeFake").as_str(),
            )?;
            let ann_infissi_dto = read_json_file::<AnnotazioneInfissoDTO>(
                path_data_fake!("annotazioniInfissiFake").as_str(),
            )?;

            {
                let mut conn = db_manager.get_connection().await?;
                for edificio_dto in edifici_dto {
                    EdificioDAO::insert(&mut conn, edificio_dto.into())?;
                }

                for stanza_dto in stanze_dto {
                    StanzaDAO::insert(&mut conn, NewStanza::from(stanza_dto))?;
                }

                for infisso_dto in infissi_dto {
                    InfissoDAO::insert(&mut conn, infisso_dto.into())?;
                }

                for ann_edificio_dto in ann_edifici_dto {
                    AnnotazioneEdificioDAO::insert(&mut conn, ann_edificio_dto.into())?;
                }

                for ann_stanza_dto in ann_stanze_dto {
                    AnnotazioneStanzaDAO::insert(&mut conn, ann_stanza_dto.into())?;
                }

                for ann_infisso_dto in ann_infissi_dto {
                    AnnotazioneInfissoDAO::insert(&mut conn, ann_infisso_dto.into())?;
                }
            }

            Ok(())
        })
        .await
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_retrieve_annotazione_edifici() -> ResultTest {
        let env = setup_env_annotazione().await?;
        let state_db = env.database();

        match <AnnotazioneService as RetrieveManyService<AnnotazioneEdificioDTO>>::retrieve_many(
            state_db,
        )
        .await
        {
            Ok(result) => {
                assert_eq!(result.len(), 11);
                println!("{:#?}", result)
            }
            Err(e) => panic!("Errore: {}", e),
        }

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_create_annotazione_edifici() -> ResultTest {
        let env = setup_env_annotazione().await?;
        let state_db = env.database();

        let insert_ann_edificio = AnnotazioneEdificioDTO {
            id: 0,
            edificio_id: "8361-122".to_string(),
            content: "TEST ANNOTAZIONE".to_string(),
        };

        match <AnnotazioneService as CreateService<AnnotazioneEdificioDTO>>::create(
            state_db,
            insert_ann_edificio,
        )
        .await
        {
            Ok(result) => {
                assert_eq!(result.id, 12)
            }
            Err(e) => panic!("Errore: {}", e),
        }

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_retrieve_annotazione_stanza() -> ResultTest {
        let env = setup_env_annotazione().await?;
        let state_db = env.database();

        match <AnnotazioneService as RetrieveManyService<AnnotazioneStanzaDTO>>::retrieve_many(
            state_db,
        )
        .await
        {
            Ok(result) => {
                assert_eq!(result.len(), 11);
                println!("{:#?}", result)
            }
            Err(e) => panic!("Errore: {}", e),
        }

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_create_annotazione_stanza() -> ResultTest {
        let env = setup_env_annotazione().await?;
        let state_db = env.database();

        let insert_ann_stanza = AnnotazioneStanzaDTO {
            id: 0,
            stanza_id: 1,
            content: "TEST ANNOTAZIONE".to_string(),
        };

        match <AnnotazioneService as CreateService<AnnotazioneStanzaDTO>>::create(
            state_db,
            insert_ann_stanza,
        )
        .await
        {
            Ok(result) => {
                assert_eq!(result.id, 12)
            }
            Err(e) => panic!("Errore: {}", e),
        }

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_retrieve_annotazione_infisso() -> ResultTest {
        let env = setup_env_annotazione().await?;
        let state_db = env.database();

        match <AnnotazioneService as RetrieveManyService<AnnotazioneInfissoDTO>>::retrieve_many(
            state_db,
        )
        .await
        {
            Ok(result) => {
                assert_eq!(result.len(), 11);
                println!("{:#?}", result)
            }
            Err(e) => panic!("Errore: {}", e),
        }

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_create_annotazione_infisso() -> ResultTest {
        let env = setup_env_annotazione().await?;
        let state_db = env.database();

        let insert_ann_stanza = AnnotazioneInfissoDTO {
            id: 0,
            infisso_id: "B".to_string(),
            edificio_id: "8361-122".to_string(),
            content: "TEST ANNOTAZIONE".to_string(),
        };

        match <AnnotazioneService as CreateService<AnnotazioneInfissoDTO>>::create(
            state_db,
            insert_ann_stanza,
        )
        .await
        {
            Ok(result) => {
                assert_eq!(result.id, 12)
            }
            Err(e) => panic!("Errore: {}", e),
        }

        Ok(())
    }
}
