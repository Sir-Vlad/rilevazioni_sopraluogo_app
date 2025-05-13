use crate::dao::crud_operations::{GetAll, Insert};
use crate::dao::{
    ClimatizzazioneDAO, IlluminazioneDAO, MaterialeInfissoDAO, TipoInfissoDAO, VetroInfissoDAO,
};
use crate::database::Database;
use crate::dto::{
    ClimatizzazioneDTO, IlluminazioneDTO, MaterialeInfissoDTO, TipoDTO, TipoInfissiDTO,
    VetroInfissoDTO, DTO,
};
use crate::service::utils::RetrieveManyService;
use crate::utils::AppError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::collections::HashMap;
use tauri::State;

#[derive(Debug, Serialize, Clone)]
pub enum TypeDTO {
    Climatizzazione,
    Illuminazione,
}

#[derive(Debug, thiserror::Error)]
#[error("Tipo non valido: {0}")]
pub struct InvalidTypeError(pub String);

impl TryFrom<String> for TypeDTO {
    type Error = InvalidTypeError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase().as_str() {
            "climatizzazione" | "riscaldamento" | "raffrescamento" => Ok(TypeDTO::Climatizzazione),
            "illuminazione" => Ok(TypeDTO::Illuminazione),
            _ => Err(InvalidTypeError(value)),
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

pub trait TypeService {
    fn retrieve_all(db: State<'_, Database>) -> Result<HashMap<String, Vec<Value>>, AppError>;
    fn insert_type(db: State<'_, Database>, dto: TipoDTO) -> Result<TipoDTO, AppError>;
}

pub struct TypeServiceImpl;

impl TypeService for TypeServiceImpl {
    fn retrieve_all(db: State<'_, Database>) -> Result<HashMap<String, Vec<Value>>, AppError> {
        let mut result_map: HashMap<String, Vec<Value>> = HashMap::new();

        let materiali: Vec<Value> =
            convert_to_json(MaterialeInfissoService::retrieve_many(db.clone())?);
        result_map.insert("materiale_infissi".to_string(), materiali);

        let vetro = convert_to_json(VetroInfissoService::retrieve_many(db.clone())?);
        result_map.insert("vetro_infissi".to_string(), vetro);

        let climatizzazione = convert_to_json(ClimatizzazioneService::retrieve_many(db.clone())?);
        result_map.insert("climatizzazione".to_string(), climatizzazione);

        let illuminazione = convert_to_json(IlluminazioneService::retrieve_many(db.clone())?);
        result_map.insert("illuminazione".to_string(), illuminazione);

        let tipo_infissi = convert_to_json(TipoInfissoService::retrieve_many(db.clone())?);
        result_map.insert("tipo_infissi".to_string(), tipo_infissi);

        Ok(result_map)
    }

    fn insert_type(db: State<'_, Database>, dto: TipoDTO) -> Result<TipoDTO, AppError> {
        let conn = db.get_conn();
        if let Some(conn) = conn.as_ref() {
            match dto.tipo {
                TypeDTO::Climatizzazione => {
                    let res = ClimatizzazioneDAO::insert(conn, dto.into())?;
                    Ok(TipoDTO::from(res))
                }
                TypeDTO::Illuminazione => {
                    let res = IlluminazioneDAO::insert(conn, dto.into())?;
                    Ok(TipoDTO::from(res))
                }
            }
        } else {
            Err(AppError::DatabaseNotInitialized)
        }
    }
}

struct MaterialeInfissoService;
impl RetrieveManyService<MaterialeInfissoDTO> for MaterialeInfissoService {
    fn retrieve_many(db: State<'_, Database>) -> Result<Vec<MaterialeInfissoDTO>, AppError> {
        let conn = db.get_conn();
        if let Some(conn) = conn.as_ref() {
            let result = MaterialeInfissoDAO::get_all(conn)?;

            Ok(result
                .iter()
                .map(|x| MaterialeInfissoDTO {
                    materiale: x.materiale.clone(),
                    efficienza_energetica: x.efficienza_energetica,
                })
                .collect())
        } else {
            Err(AppError::DatabaseNotInitialized)
        }
    }
}

struct VetroInfissoService;

impl RetrieveManyService<VetroInfissoDTO> for VetroInfissoService {
    fn retrieve_many(db: State<'_, Database>) -> Result<Vec<VetroInfissoDTO>, AppError> {
        let conn = db.get_conn();
        if let Some(conn) = conn.as_ref() {
            let result = VetroInfissoDAO::get_all(conn)?;

            Ok(result
                .iter()
                .map(|x| VetroInfissoDTO {
                    vetro: x.vetro.clone(),
                    efficienza_energetica: x.efficienza_energetica,
                })
                .collect())
        } else {
            Err(AppError::DatabaseNotInitialized)
        }
    }
}

struct ClimatizzazioneService;

impl RetrieveManyService<ClimatizzazioneDTO> for ClimatizzazioneService {
    fn retrieve_many(db: State<'_, Database>) -> Result<Vec<ClimatizzazioneDTO>, AppError> {
        let conn = db.get_conn();
        if let Some(conn) = conn.as_ref() {
            let result = ClimatizzazioneDAO::get_all(conn)?;

            Ok(result
                .iter()
                .map(|x| ClimatizzazioneDTO {
                    climatizzazione: x.climatizzazione.clone(),
                    efficienza_energetica: x.efficienza_energetica,
                })
                .collect())
        } else {
            Err(AppError::DatabaseNotInitialized)
        }
    }
}

struct IlluminazioneService;

impl RetrieveManyService<IlluminazioneDTO> for IlluminazioneService {
    fn retrieve_many(db: State<'_, Database>) -> Result<Vec<IlluminazioneDTO>, AppError> {
        let conn = db.get_conn();
        if let Some(conn) = conn.as_ref() {
            let result = IlluminazioneDAO::get_all(conn)?;

            Ok(result
                .iter()
                .map(|x| IlluminazioneDTO {
                    lampadina: x.lampadina.clone(),
                    efficienza_energetica: x.efficienza_energetica,
                })
                .collect())
        } else {
            Err(AppError::DatabaseNotInitialized)
        }
    }
}

struct TipoInfissoService;

impl RetrieveManyService<TipoInfissiDTO> for TipoInfissoService {
    fn retrieve_many(db: State<'_, Database>) -> Result<Vec<TipoInfissiDTO>, AppError> {
        let conn = db.get_conn();
        if let Some(conn) = conn.as_ref() {
            let result = TipoInfissoDAO::get_all(conn)?;

            Ok(result
                .iter()
                .map(|x| TipoInfissiDTO {
                    nome: x.nome.clone(),
                })
                .collect())
        } else {
            Err(AppError::DatabaseNotInitialized)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::database::Database;
    use crate::dto::TipoDTO;
    use crate::service::{TypeDTO, TypeService, TypeServiceImpl};
    use tauri::test::MockRuntime;
    use tauri::{App, Manager};

    fn setup() -> App<MockRuntime> {
        let app = tauri::test::mock_app();
        let db = Database::open_in_memory();
        crate::dao::create_tables(db.get_conn().as_ref().unwrap()).expect("create tables");
        app.manage(db);
        app
    }

    #[test]
    fn test_insert_type() {
        let app = setup();
        let dto = TipoDTO {
            tipo: TypeDTO::Climatizzazione,
            name: "Vetro".to_string(),
            efficienza_energetica: 1,
        };

        let res = TypeServiceImpl::insert_type(app.state::<Database>(), dto);
        match res {
            Ok(r) => {
                println!("{:?}", r)
            }
            Err(e) => {
                panic!("{}", e)
            }
        }
    }
}
