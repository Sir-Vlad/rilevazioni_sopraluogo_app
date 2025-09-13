use crate::dao::{
    ClimatizzazioneDAO, IlluminazioneDAO, MaterialeInfissoDAO, TipoInfissoDAO, VetroInfissoDAO,
};
use crate::dto::{
    ClimatizzazioneDTO, IlluminazioneDTO, MaterialeInfissoDTO, TipoDTO, TipoInfissiDTO,
    VetroInfissoDTO,
};
use crate::service::DomainError;
use app_utils::app_error::{AppResult, ApplicationError};
use app_utils::app_interface::dao_interface::crud_operations::{GetAll, Insert};
use app_utils::app_interface::database_interface::DatabaseManagerTrait;
use app_utils::app_interface::dto_interface::DTO;
use app_utils::app_interface::service_interface::RetrieveManyService;
use async_trait::async_trait;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tauri::State;

#[derive(Debug, Serialize, Clone)]
pub enum TypeDTO {
    Climatizzazione,
    Illuminazione,
}

impl TryFrom<String> for TypeDTO {
    type Error = DomainError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase().as_str() {
            "climatizzazione" | "riscaldamento" | "raffrescamento" => Ok(TypeDTO::Climatizzazione),
            "illuminazione" => Ok(TypeDTO::Illuminazione),
            _ => Err(DomainError::TipoInvalid(value)),
        }
    }
}

impl<'de> Deserialize<'de> for TypeDTO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match TypeDTO::try_from(s) {
            Ok(t) => Ok(t),
            Err(e) => Err(serde::de::Error::custom(e.to_string())),
        }
    }
}

fn convert_to_json<T: DTO + Serialize>(items: Vec<T>) -> Vec<Value> {
    items
        .iter()
        .map(|x| serde_json::to_value(x).unwrap_or_default())
        .collect()
}

#[async_trait]
pub trait TypeService {
    async fn retrieve_all(
        db: State<'_, impl DatabaseManagerTrait + Send + Sync>,
    ) -> Result<HashMap<String, Vec<Value>>, ApplicationError>;
    async fn insert_type(
        db: State<'_, impl DatabaseManagerTrait + Send + Sync>,
        dto: TipoDTO,
    ) -> Result<TipoDTO, ApplicationError>;
}

pub struct TypeServiceImpl;

#[async_trait]
impl TypeService for TypeServiceImpl {
    async fn retrieve_all(
        db: State<'_, impl DatabaseManagerTrait + Send + Sync>,
    ) -> AppResult<HashMap<String, Vec<Value>>> {
        let mut result_map: HashMap<String, Vec<Value>> = HashMap::new();

        let materiali: Vec<Value> =
            convert_to_json(MaterialeInfissoService::retrieve_many(db.clone()).await?);
        result_map.insert("materiale_infissi".to_string(), materiali);

        let vetro = convert_to_json(VetroInfissoService::retrieve_many(db.clone()).await?);
        result_map.insert("vetro_infissi".to_string(), vetro);

        let climatizzazione =
            convert_to_json(ClimatizzazioneService::retrieve_many(db.clone()).await?);
        result_map.insert("climatizzazione".to_string(), climatizzazione);

        let illuminazione = convert_to_json(IlluminazioneService::retrieve_many(db.clone()).await?);
        result_map.insert("illuminazione".to_string(), illuminazione);

        let tipo_infissi = convert_to_json(TipoInfissoService::retrieve_many(db.clone()).await?);
        result_map.insert("tipo_infissi".to_string(), tipo_infissi);

        Ok(result_map)
    }

    async fn insert_type(
        db: State<'_, impl DatabaseManagerTrait + Send + Sync>,
        dto: TipoDTO,
    ) -> Result<TipoDTO, ApplicationError> {
        let mut conn = db.get_connection().await?;
        match dto.tipo {
            TypeDTO::Climatizzazione => {
                let res = ClimatizzazioneDAO::insert(&mut conn, dto.into())?;
                Ok(TipoDTO::from(res))
            }
            TypeDTO::Illuminazione => {
                let res = IlluminazioneDAO::insert(&mut conn, dto.into())?;
                Ok(TipoDTO::from(res))
            }
        }
    }
}

struct MaterialeInfissoService;

#[async_trait]
impl RetrieveManyService<MaterialeInfissoDTO> for MaterialeInfissoService {
    async fn retrieve_many(
        db: State<'_, impl DatabaseManagerTrait + Send + Sync>,
    ) -> Result<Vec<MaterialeInfissoDTO>, ApplicationError> {
        let mut conn = db.get_connection().await?;
        let result = MaterialeInfissoDAO::get_all(&mut conn)?;

        Ok(result
            .iter()
            .map(|x| MaterialeInfissoDTO {
                materiale: x.materiale.clone(),
                efficienza_energetica: x.eff_energetica as u8,
            })
            .collect())
    }
}

struct VetroInfissoService;

#[async_trait]
impl RetrieveManyService<VetroInfissoDTO> for VetroInfissoService {
    async fn retrieve_many(
        db: State<'_, impl DatabaseManagerTrait + Send + Sync>,
    ) -> Result<Vec<VetroInfissoDTO>, ApplicationError> {
        let mut conn = db.get_connection().await?;
        let result = VetroInfissoDAO::get_all(&mut conn)?;

        Ok(result
            .iter()
            .map(|x| VetroInfissoDTO {
                vetro: x.vetro.clone(),
                efficienza_energetica: x.eff_energetica as u8,
            })
            .collect())
    }
}

struct ClimatizzazioneService;

#[async_trait]
impl RetrieveManyService<ClimatizzazioneDTO> for ClimatizzazioneService {
    async fn retrieve_many(
        db: State<'_, impl DatabaseManagerTrait + Send + Sync>,
    ) -> Result<Vec<ClimatizzazioneDTO>, ApplicationError> {
        let mut conn = db.get_connection().await?;
        let result = ClimatizzazioneDAO::get_all(&mut conn)?;

        Ok(result
            .iter()
            .map(|x| ClimatizzazioneDTO {
                climatizzazione: x.nome.clone(),
                efficienza_energetica: x.eff_energetica as u8,
            })
            .collect())
    }
}

struct IlluminazioneService;

#[async_trait]
impl RetrieveManyService<IlluminazioneDTO> for IlluminazioneService {
    async fn retrieve_many(
        db: State<'_, impl DatabaseManagerTrait + Send + Sync>,
    ) -> Result<Vec<IlluminazioneDTO>, ApplicationError> {
        let mut conn = db.get_connection().await?;
        let result = IlluminazioneDAO::get_all(&mut conn)?;

        Ok(result
            .iter()
            .map(|x| IlluminazioneDTO {
                lampadina: x.lampadina.clone(),
                efficienza_energetica: x.eff_energetica as u8,
            })
            .collect())
    }
}

struct TipoInfissoService;

#[async_trait]
impl RetrieveManyService<TipoInfissiDTO> for TipoInfissoService {
    async fn retrieve_many(
        db: State<'_, impl DatabaseManagerTrait + Send + Sync>,
    ) -> Result<Vec<TipoInfissiDTO>, ApplicationError> {
        let mut conn = db.get_connection().await?;
        let result = TipoInfissoDAO::get_all(&mut conn)?;

        Ok(result
            .iter()
            .map(|x| TipoInfissiDTO {
                nome: x.nome.clone(),
            })
            .collect())
    }
}

#[cfg(test)]
mod test {
    use crate::dto::TipoDTO;
    use crate::service::{TypeDTO, TypeService, TypeServiceImpl};
    use app_state::database::DatabaseManager;
    use app_utils::test::{ResultTest, TestServiceEnvironment};

    async fn setup_env_type() -> ResultTest<TestServiceEnvironment<DatabaseManager>> {
        TestServiceEnvironment::new::<_, _>(|_db_manager: DatabaseManager| async { Ok(()) }).await
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_retrieve_type() -> ResultTest {
        let env = setup_env_type().await?;
        let state_db = env.database();

        match TypeServiceImpl::retrieve_all(state_db).await {
            Ok(result) => {
                println!("{:?}", result);
            }
            Err(e) => panic!("{:?}", e),
        }
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_insert_new_climatizzazione_type() -> ResultTest {
        let env = setup_env_type().await?;
        let state_db = env.database();

        let insert_type = TipoDTO {
            tipo: TypeDTO::Climatizzazione,
            name: "TEST".to_string(),
            eff_energetica: 100,
        };

        match TypeServiceImpl::insert_type(state_db, insert_type).await {
            Ok(result) => {
                println!("{:?}", result);
            }
            Err(e) => panic!("{:?}", e),
        }

        Ok(())
    }


    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_insert_new_illuminazione_type() -> ResultTest {
        let env = setup_env_type().await?;
        let state_db = env.database();

        let insert_type = TipoDTO {
            tipo: TypeDTO::Illuminazione,
            name: "TEST".to_string(),
            eff_energetica: 100,
        };

        match TypeServiceImpl::insert_type(state_db, insert_type).await {
            Ok(result) => {
                println!("{:?}", result);
            }
            Err(e) => panic!("{:?}", e),
        }

        Ok(())
    }


    #[test]
    #[should_panic(expected = "TipoInvalid(\"MaterialeInfisso\")")]
    fn test_type_not_support() {
        let type_str = "MaterialeInfisso".to_string();
        TypeDTO::try_from(type_str).unwrap();
    }
}
