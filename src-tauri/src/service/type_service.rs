use crate::dao::{
    ClimatizzazioneDao, ClimatizzazioneDaoImpl, IlluminazioneDao, IlluminazioneDaoImpl,
    MaterialeInfissoDao, MaterialeInfissoDaoImpl, VetroInfissoDao, VetroInfissoDaoImpl,
};
use crate::database::Database;
use crate::dto::{ClimatizzazioneDto, IlluminazioneDto, MaterialeInfissoDto, VetroInfissoDto};
use serde_json::Value;
use std::collections::HashMap;
use tauri::State;

pub trait TypeService {
    fn get_all(db: State<'_, Database>) -> Result<HashMap<String, Vec<Value>>, String>;
    fn get_materiale_infisso(db: State<'_, Database>) -> Result<Vec<MaterialeInfissoDto>, String>;
    fn get_vetro_infisso(db: State<'_, Database>) -> Result<Vec<VetroInfissoDto>, String>;
    fn get_climatizzazione(db: State<'_, Database>) -> Result<Vec<ClimatizzazioneDto>, String>;
    fn get_illuminazione(db: State<'_, Database>) -> Result<Vec<IlluminazioneDto>, String>;
}

pub struct TypeServiceImpl;

impl TypeService for TypeServiceImpl {
    fn get_all(db: State<'_, Database>) -> Result<HashMap<String, Vec<Value>>, String> {
        let conn = db.get_conn();

        if let Some(conn) = conn.as_ref() {
            let mut result_map: HashMap<String, Vec<Value>> = HashMap::new();

            let materiali = MaterialeInfissoDaoImpl::get_all(conn)?;
            let materiale_json: Vec<Value> = materiali
                .iter()
                .map(|x| MaterialeInfissoDto {
                    materiale: x.materiale.clone(),
                    efficienza_energetica: x.efficienza_energetica,
                })
                .map(|x| serde_json::to_value(x).unwrap_or_default())
                .collect();

            result_map.insert("materiale_infissi".to_string(), materiale_json);

            let vetro = VetroInfissoDaoImpl::get_all(conn)?;
            let vetro_json: Vec<Value> = vetro
                .iter()
                .map(|x| VetroInfissoDto {
                    vetro: x.vetro.clone(),
                    efficienza_energetica: x.efficienza_energetica,
                })
                .map(|x| serde_json::to_value(x).unwrap_or_default())
                .collect();

            result_map.insert("vetro_infissi".to_string(), vetro_json);

            let climatizzazione = ClimatizzazioneDaoImpl::get_all(conn)?;
            let climatizzazione_json: Vec<Value> = climatizzazione
                .iter()
                .map(|x| ClimatizzazioneDto {
                    climatizzazione: x.climatizzazione.clone(),
                    efficienza_energetica: x.efficienza_energetica,
                })
                .map(|x| serde_json::to_value(x).unwrap_or_default())
                .collect();

            result_map.insert("climatizzazione".to_string(), climatizzazione_json);

            let illuminazione = IlluminazioneDaoImpl::get_all(conn)?;
            let illuminazione_json: Vec<Value> = illuminazione
                .iter()
                .map(|x| IlluminazioneDto {
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

    fn get_materiale_infisso(db: State<'_, Database>) -> Result<Vec<MaterialeInfissoDto>, String> {
        let conn = db.get_conn();
        let result = MaterialeInfissoDaoImpl::get_all(conn.as_ref().unwrap())?;

        Ok(result
            .iter()
            .map(|x| MaterialeInfissoDto {
                materiale: x.materiale.clone(),
                efficienza_energetica: x.efficienza_energetica,
            })
            .collect())
    }

    fn get_vetro_infisso(db: State<'_, Database>) -> Result<Vec<VetroInfissoDto>, String> {
        let conn = db.get_conn();
        let result = VetroInfissoDaoImpl::get_all(conn.as_ref().unwrap())?;

        Ok(result
            .iter()
            .map(|x| VetroInfissoDto {
                vetro: x.vetro.clone(),
                efficienza_energetica: x.efficienza_energetica,
            })
            .collect())
    }

    fn get_climatizzazione(db: State<'_, Database>) -> Result<Vec<ClimatizzazioneDto>, String> {
        let conn = db.get_conn();
        let result = ClimatizzazioneDaoImpl::get_all(conn.as_ref().unwrap())?;

        Ok(result
            .iter()
            .map(|x| ClimatizzazioneDto {
                climatizzazione: x.climatizzazione.clone(),
                efficienza_energetica: x.efficienza_energetica,
            })
            .collect())
    }

    fn get_illuminazione(db: State<'_, Database>) -> Result<Vec<IlluminazioneDto>, String> {
        let conn = db.get_conn();
        let result = IlluminazioneDaoImpl::get_all(conn.as_ref().unwrap())?;

        Ok(result
            .iter()
            .map(|x| IlluminazioneDto {
                lampadina: x.lampadina.clone(),
                efficienza_energetica: x.efficienza_energetica,
            })
            .collect())
    }
}
