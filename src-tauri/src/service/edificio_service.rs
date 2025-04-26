use crate::dao::{EdificioDAO, EdificioDAOImpl};
use crate::database::Database;
use crate::dto::EdificioDTO;
use tauri::State;

pub trait EdificioService {
    fn get_all(db: State<'_, Database>) -> Result<Vec<EdificioDTO>, String>;
    fn update(db: State<'_, Database>, edificio: EdificioDTO) -> Result<EdificioDTO, String>;
}

pub struct EdificioServiceImpl;

impl EdificioService for EdificioServiceImpl {
    fn get_all(db: State<'_, Database>) -> Result<Vec<EdificioDTO>, String> {
        let conn = db.get_conn();
        if let Some(conn) = conn.as_ref() {
            let result = EdificioDAOImpl::get_all(conn)?;
            Ok(result.iter().map(EdificioDTO::from).collect())
        } else {
            Err("Database not initialized".to_string())
        }
    }

    fn update(db: State<'_, Database>, edificio: EdificioDTO) -> Result<EdificioDTO, String> {
        let conn = db.get_conn();
        if let Some(conn) = conn.as_ref() {
            let result = EdificioDAOImpl::update(conn, edificio.into())?;
            Ok(EdificioDTO::from(&result))
        } else {
            Err("Database not initialized".to_string())
        }
    }
}
