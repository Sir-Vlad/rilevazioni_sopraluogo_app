use crate::dao::crud_operations::GetAll;
use crate::dao::{ClimatizzazioneDAO, IlluminazioneDAO, MaterialeInfissoDAO, VetroInfissoDAO};
use crate::database::Database;
use crate::dto::{ClimatizzazioneDTO, IlluminazioneDTO, MaterialeInfissoDTO, VetroInfissoDTO, DTO};
use crate::service::utils::RetrieveManyService;
use crate::utils::AppError;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use tauri::State;

fn convert_to_json<T: DTO + Serialize>(items: Vec<T>) -> Vec<Value> {
    items
        .iter()
        .map(|x| serde_json::to_value(x).unwrap_or_default())
        .collect()
}

pub trait TypeService {
    fn retrieve_all(db: State<'_, Database>) -> Result<HashMap<String, Vec<Value>>, AppError>;
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

        Ok(result_map)
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
