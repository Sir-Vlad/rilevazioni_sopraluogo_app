use crate::dao::{
    ClimatizzazioneDAO, ClimatizzazioneDAOImpl, IlluminazioneDAO, IlluminazioneDAOImpl,
    MaterialeInfissoDAO, MaterialeInfissoDAOImpl, VetroInfissoDAO, VetroInfissoDAOImpl,
};
use crate::database::Database;
use crate::dto::{ClimatizzazioneDTO, IlluminazioneDTO, MaterialeInfissoDTO, VetroInfissoDTO};
use serde_json::Value;
use std::collections::HashMap;
use tauri::State;

pub trait TypeService {
    fn get_all(db: State<'_, Database>) -> Result<HashMap<String, Vec<Value>>, String>;
    fn get_materiale_infisso(db: State<'_, Database>) -> Result<Vec<MaterialeInfissoDTO>, String>;
    fn get_vetro_infisso(db: State<'_, Database>) -> Result<Vec<VetroInfissoDTO>, String>;
    fn get_climatizzazione(db: State<'_, Database>) -> Result<Vec<ClimatizzazioneDTO>, String>;
    fn get_illuminazione(db: State<'_, Database>) -> Result<Vec<IlluminazioneDTO>, String>;
}

pub struct TypeServiceImpl;

impl TypeService for TypeServiceImpl {
    fn get_all(db: State<'_, Database>) -> Result<HashMap<String, Vec<Value>>, String> {
        let conn = db.get_conn();

        if let Some(conn) = conn.as_ref() {
            let mut result_map: HashMap<String, Vec<Value>> = HashMap::new();

            let materiali = MaterialeInfissoDAOImpl::get_all(conn)?;
            let materiale_json: Vec<Value> = materiali
                .iter()
                .map(|x| MaterialeInfissoDTO {
                    materiale: x.materiale.clone(),
                    efficienza_energetica: x.efficienza_energetica,
                })
                .map(|x| serde_json::to_value(x).unwrap_or_default())
                .collect();

            result_map.insert("materiale_infissi".to_string(), materiale_json);

            let vetro = VetroInfissoDAOImpl::get_all(conn)?;
            let vetro_json: Vec<Value> = vetro
                .iter()
                .map(|x| VetroInfissoDTO {
                    vetro: x.vetro.clone(),
                    efficienza_energetica: x.efficienza_energetica,
                })
                .map(|x| serde_json::to_value(x).unwrap_or_default())
                .collect();

            result_map.insert("vetro_infissi".to_string(), vetro_json);

            let climatizzazione = ClimatizzazioneDAOImpl::get_all(conn)?;
            let climatizzazione_json: Vec<Value> = climatizzazione
                .iter()
                .map(|x| ClimatizzazioneDTO {
                    climatizzazione: x.climatizzazione.clone(),
                    efficienza_energetica: x.efficienza_energetica,
                })
                .map(|x| serde_json::to_value(x).unwrap_or_default())
                .collect();

            result_map.insert("climatizzazione".to_string(), climatizzazione_json);

            let illuminazione = IlluminazioneDAOImpl::get_all(conn)?;
            let illuminazione_json: Vec<Value> = illuminazione
                .iter()
                .map(|x| IlluminazioneDTO {
                    lampadina: x.lampadina.clone(),
                    efficienza_energetica: x.efficienza_energetica,
                })
                .map(|x| serde_json::to_value(x).unwrap_or_default())
                .collect();
            result_map.insert("illuminazione".to_string(), illuminazione_json);

            Ok(result_map)
        } else {
            Err("Database non inizializzato".to_string())
        }
    }

    fn get_materiale_infisso(db: State<'_, Database>) -> Result<Vec<MaterialeInfissoDTO>, String> {
        let conn = db.get_conn();
        let result = MaterialeInfissoDAOImpl::get_all(conn.as_ref().unwrap())?;

        Ok(result
            .iter()
            .map(|x| MaterialeInfissoDTO {
                materiale: x.materiale.clone(),
                efficienza_energetica: x.efficienza_energetica,
            })
            .collect())
    }

    fn get_vetro_infisso(db: State<'_, Database>) -> Result<Vec<VetroInfissoDTO>, String> {
        let conn = db.get_conn();
        let result = VetroInfissoDAOImpl::get_all(conn.as_ref().unwrap())?;

        Ok(result
            .iter()
            .map(|x| VetroInfissoDTO {
                vetro: x.vetro.clone(),
                efficienza_energetica: x.efficienza_energetica,
            })
            .collect())
    }

    fn get_climatizzazione(db: State<'_, Database>) -> Result<Vec<ClimatizzazioneDTO>, String> {
        let conn = db.get_conn();
        let result = ClimatizzazioneDAOImpl::get_all(conn.as_ref().unwrap())?;

        Ok(result
            .iter()
            .map(|x| ClimatizzazioneDTO {
                climatizzazione: x.climatizzazione.clone(),
                efficienza_energetica: x.efficienza_energetica,
            })
            .collect())
    }

    fn get_illuminazione(db: State<'_, Database>) -> Result<Vec<IlluminazioneDTO>, String> {
        let conn = db.get_conn();
        let result = IlluminazioneDAOImpl::get_all(conn.as_ref().unwrap())?;

        Ok(result
            .iter()
            .map(|x| IlluminazioneDTO {
                lampadina: x.lampadina.clone(),
                efficienza_energetica: x.efficienza_energetica,
            })
            .collect())
    }
}
