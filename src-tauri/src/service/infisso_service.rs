use crate::dao::{InfissoDao, InfissoDaoImpl};
use crate::database::Database;
use crate::dto::InfissoDto;
use tauri::State;

pub trait InfissoService {
    fn get_all(db: State<'_, Database>) -> Result<Vec<InfissoDto>, String>;
    fn insert(db: State<'_, Database>, infisso: InfissoDto) -> Result<InfissoDto, String>;
    fn update(db: State<'_, Database>, infisso: InfissoDto) -> Result<InfissoDto, String>;
}

pub struct InfissoServiceImpl;

impl InfissoService for InfissoServiceImpl {
    fn get_all(db: State<'_, Database>) -> Result<Vec<InfissoDto>, String> {
        let conn = db.get_conn();
        if let Some(conn) = conn.as_ref() {
            let result = InfissoDaoImpl::get_all(conn)?;
            Ok(result.iter().map(InfissoDto::from).collect())
        } else {
            Err("Database not initialized".to_string())
        }
    }

    fn insert(db: State<'_, Database>, infisso: InfissoDto) -> Result<InfissoDto, String> {
        let conn = db.get_conn();
        if let Some(conn) = conn.as_ref() {
            let result = InfissoDaoImpl::insert(conn, &infisso)?;
            Ok(InfissoDto::from(&result))
        } else {
            Err("Database not initialized".to_string())
        }
    }

    fn update(db: State<'_, Database>, infisso: InfissoDto) -> Result<InfissoDto, String> {
        let conn = db.get_conn();
        if let Some(conn) = conn.as_ref() {
            let result = InfissoDaoImpl::update(conn, &infisso)?;
            Ok(InfissoDto::from(&result))
        } else {
            Err("Database not initialized".to_string())
        }
    }
}
