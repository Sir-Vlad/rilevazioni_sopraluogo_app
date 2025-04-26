use crate::dao::{InfissoDAO, InfissoDAOImpl};
use crate::database::Database;
use crate::dto::InfissoDTO;
use tauri::State;

pub trait InfissoService {
    fn get_all(db: State<'_, Database>) -> Result<Vec<InfissoDTO>, String>;
    fn insert(db: State<'_, Database>, infisso: InfissoDTO) -> Result<InfissoDTO, String>;
    fn update(db: State<'_, Database>, infisso: InfissoDTO) -> Result<InfissoDTO, String>;
}

pub struct InfissoServiceImpl;

impl InfissoService for InfissoServiceImpl {
    fn get_all(db: State<'_, Database>) -> Result<Vec<InfissoDTO>, String> {
        let conn = db.get_conn();
        if let Some(conn) = conn.as_ref() {
            let result = InfissoDAOImpl::get_all(conn)?;
            Ok(result.iter().map(InfissoDTO::from).collect())
        } else {
            Err("Database not initialized".to_string())
        }
    }

    fn insert(db: State<'_, Database>, infisso: InfissoDTO) -> Result<InfissoDTO, String> {
        let conn = db.get_conn();
        if let Some(conn) = conn.as_ref() {
            let result = InfissoDAOImpl::insert(conn, &infisso)?;
            Ok(InfissoDTO::from(&result))
        } else {
            Err("Database not initialized".to_string())
        }
    }

    fn update(db: State<'_, Database>, infisso: InfissoDTO) -> Result<InfissoDTO, String> {
        let conn = db.get_conn();
        if let Some(conn) = conn.as_ref() {
            let result = InfissoDAOImpl::update(conn, &infisso)?;
            Ok(InfissoDTO::from(&result))
        } else {
            Err("Database not initialized".to_string())
        }
    }
}
