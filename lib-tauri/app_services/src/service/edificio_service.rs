use crate::dao::EdificioDAO;
use crate::dto::EdificioDTO;
use app_error::{AppResult, ApplicationError};
use app_interface::{
    dao_interface::crud_operations::{Get, GetAll, Insert, Update},
    database_interface::DatabaseManager,
    service_interface::{
        CreateService, RetrieveManyService, RetrieveOneService, UpdateService,
    },
};
use app_state::selected_edificio::StateEdificioSelected;
use async_trait::async_trait;
use tauri::State;

pub struct EdificioService;

impl EdificioService {
    pub async fn select_edificio(stato: State<'_, StateEdificioSelected>, chiave: String) {
        let mut stato_lock = stato.write().await;
        stato_lock.set_chiave(chiave);
    }

    pub async fn get_edificio(stato: State<'_, StateEdificioSelected>) -> AppResult<String> {
        let stato_lock = stato.read().await;
        match stato_lock.get_chiave() {
            Some(stato) => Ok(stato),
            None => Err(ApplicationError::EdificioNotSelected),
        }
    }
}

#[async_trait]
impl RetrieveManyService<EdificioDTO> for EdificioService {
    async fn retrieve_many(
        db: State<'_, impl DatabaseManager + Send + Sync>,
    ) -> Result<Vec<EdificioDTO>, ApplicationError> {
        let mut conn = db.get_connection().await?;
        let result = EdificioDAO::get_all(&mut conn)?;
        Ok(result.iter().map(EdificioDTO::from).collect())
    }
}

#[async_trait]
impl RetrieveOneService<EdificioDTO, String> for EdificioService {
    async fn retrieve_one(
        db: State<'_, impl DatabaseManager + Send + Sync>,
        id: String,
    ) -> AppResult<EdificioDTO> {
        let mut conn = db.get_connection().await?;
        let result = EdificioDAO::get(&mut conn, id)?;
        Ok(EdificioDTO::from(&result))
    }
}

#[async_trait]
impl CreateService<EdificioDTO> for EdificioService {
    async fn create(
        db: State<'_, impl DatabaseManager + Send + Sync>,
        item: EdificioDTO,
    ) -> AppResult<EdificioDTO> {
        let mut conn = db.get_connection().await?;
        let result = EdificioDAO::insert(&mut conn, item.into())?;
        Ok(EdificioDTO::from(&result))
    }
}

#[async_trait]
impl UpdateService<EdificioDTO> for EdificioService {
    async fn update(
        db: State<'_, impl DatabaseManager + Send + Sync>,
        edificio: EdificioDTO,
    ) -> Result<EdificioDTO, ApplicationError> {
        let mut conn = db.get_connection().await?;
        let result = EdificioDAO::update(&mut conn, edificio.chiave.clone(), edificio.into())?;
        Ok(EdificioDTO::from(&result))
    }
}


#[cfg(test)]
mod tests {
    //! The tests were created based on the data in the `dataFake` folder.

    use crate::dao::EdificioDAO;
    use crate::dto::EdificioDTO;
    use crate::service::EdificioService;
    use app_interface::dao_interface::crud_operations::Insert;
    use app_interface::database_interface::DatabaseManager as DatabaseManagerInterface;
    use app_interface::service_interface::{CreateService, RetrieveManyService, RetrieveOneService, UpdateService};
    use app_state::database::DatabaseManager;
    use app_utils::test::{read_json_file, TestServiceEnvironment};
    use std::error::Error;

    const FILE_PATH_DATA_FAKE: &str = "../dataFake/edificiFake.json";

    async fn setup_env_edifici() -> Result<TestServiceEnvironment<DatabaseManager>, Box<dyn Error>> {
        TestServiceEnvironment::new::<_, _>(|db_manager: DatabaseManager| async move {
            let data = read_json_file::<EdificioDTO>(FILE_PATH_DATA_FAKE)?;
            {
                let mut pool = db_manager.get_connection().await?;
                for edificio in data {
                    // Ignora errori di duplicati nei test
                    let _ = EdificioDAO::insert(&mut pool, edificio.into());
                }
            }
            Ok(())
        }).await
    }


    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_retrieve_many() -> Result<(), Box<dyn Error>> {
        let env = setup_env_edifici().await?; // Setup automatico
        let state = env.database();

        match EdificioService::retrieve_many(state).await {
            Ok(result) => {
                assert!(!result.is_empty(), "Dovrebbero esserci degli edifici");
                println!("Test retrieve_many passed: {} edifici trovati", result.len());
            }
            Err(e) => panic!("Errore durante il recupero degli edifici: {}", e),
        }

        // Cleanup automatico quando `env` va out of scope
        Ok(())
    }


    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_retrieve_one() -> Result<(), Box<dyn Error>> {
        let env = setup_env_edifici().await?;
        let state = env.database();

        let chiave = "8351-198";
        match EdificioService::retrieve_one(state.clone(), chiave.to_string()).await {
            Ok(result) => {
                assert_eq!(result.chiave, chiave, "La chiave dovrebbe corrispondere");
                assert_eq!(
                    result.fascicolo, 6025,
                    "Il fascicolo dovrebbe corrispondere"
                );
                println!("Test retrieve_one passed per edificio: {}", result.chiave);
            }
            Err(e) => panic!("Errore durante il recupero dell'edificio: {}", e),
        }
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_create() -> Result<(), Box<dyn Error>> {
        let env = setup_env_edifici().await?;
        let state = env.database();

        let nuovo_edificio = EdificioDTO {
            chiave: "TEST_001".to_string(),
            fascicolo: 9999,
            indirizzo: "Via Test 123".to_string(),
            anno_costruzione: None,
            anno_riqualificazione: None,
            note_riqualificazione: None,
            isolamento_tetto: false,
            cappotto: false,
        };

        match EdificioService::create(state, nuovo_edificio).await {
            Ok(result) => {
                assert_eq!(
                    result.chiave, "TEST_001",
                    "La chiave dovrebbe corrispondere"
                );
                assert_eq!(
                    result.indirizzo, "Via Test 123",
                    "L'indirizzo dovrebbe corrispondere"
                );

                println!("Test create passed per edificio: {}", result.chiave);
                println!("Edificio creato: {result:#?}");
            }
            Err(e) => panic!("Errore durante la creazione dell'edificio: {}", e),
        }
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_update() -> Result<(), Box<dyn Error>> {
        let env = setup_env_edifici().await?;
        let state = env.database();

        let edificio_dto = EdificioDTO {
            chiave: "8351-198".to_string(),
            fascicolo: 6025,
            indirizzo: "7010 E 3rd Street".to_string(),
            anno_costruzione: Some(1950),
            anno_riqualificazione: Some(2025),
            note_riqualificazione: Some("Nota di test".to_string()),
            isolamento_tetto: false,
            cappotto: false,
        };

        match EdificioService::update(state, edificio_dto).await {
            Ok(result) => {
                assert_eq!(
                    result.chiave, "8351-198",
                    "La chiave dovrebbe corrispondere"
                );
                assert_eq!(
                    result.indirizzo, "7010 E 3rd Street",
                    "L'indirizzo dovrebbe corrispondere"
                );
                assert_eq!(
                    result.anno_costruzione,
                    Some(1950),
                    "L'anno di costruzione dovrebbe corrispondere"
                );
                assert_eq!(
                    result.anno_riqualificazione,
                    Some(2025),
                    "L'anno di riqualificazione dovrebbe corrispondere"
                );
                assert_eq!(
                    result.note_riqualificazione,
                    Some("Nota di test".to_string()),
                    "La nota di riqualificazione dovrebbe corrispondere"
                );
                assert!(
                    !result.isolamento_tetto,
                    "L'isolamento tetto dovrebbe essere false"
                );
                assert!(!result.cappotto, "Il cappotto dovrebbe essere false");
                println!("Test update passed per edificio: {}", result.chiave);
                println!("Edificio aggiornato: {result:#?}");
            }
            Err(e) => panic!("Errore durante l'aggiornamento dell'edificio: {}", e),
        }
        Ok(())
    }
}
