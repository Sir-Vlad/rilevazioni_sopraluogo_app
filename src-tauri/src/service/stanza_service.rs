use crate::dao::{StanzaDao, StanzaDaoImpl};
use crate::database::Database;
use crate::dto::StanzaDto;
use tauri::State;
pub trait StanzaService {
    fn get_all(db: State<'_, Database>) -> Result<Vec<StanzaDto>, String>;
    fn insert(db: State<'_, Database>, stanza: StanzaDto) -> Result<StanzaDto, String>;
    fn update(db: State<'_, Database>, stanza: StanzaDto) -> Result<StanzaDto, String>;
    fn get_with_infissi(db: State<'_, Database>, id: i64) -> Result<Vec<StanzaDto>, String>;
    fn insert_with_infissi(db: State<'_, Database>, stanza: StanzaDto)
        -> Result<StanzaDto, String>;
}

pub struct StanzaServiceImpl;

impl StanzaService for StanzaServiceImpl {
    fn get_all(db: State<'_, Database>) -> Result<Vec<StanzaDto>, String> {
        let conn = db.get_conn();
        if let Some(conn) = conn.as_ref() {
            let result = StanzaDaoImpl::get_all(conn)?;
            Ok(result.iter().map(StanzaDto::from).collect())
        } else {
            Err("Database not initialized".to_string())
        }
    }

    fn insert(db: State<'_, Database>, stanza: StanzaDto) -> Result<StanzaDto, String> {
        let conn = db.get_conn();
        if let Some(conn) = conn.as_ref() {
            let result = StanzaDaoImpl::insert(conn, stanza)?;
            Ok(StanzaDto::from(&result))
        } else {
            Err("Database not initialized".to_string())
        }
    }

    fn update(db: State<'_, Database>, stanza: StanzaDto) -> Result<StanzaDto, String> {
        let conn = db.get_conn();
        if let Some(conn) = conn.as_ref() {
            let result = StanzaDaoImpl::update(conn, stanza)?;
            Ok(StanzaDto::from(&result))
        } else {
            Err("Database not initialized".to_string())
        }
    }

    #[allow(dead_code)]
    fn get_with_infissi(db: State<'_, Database>, id: i64) -> Result<Vec<StanzaDto>, String> {
        todo!()
    }
    #[allow(dead_code)]
    fn insert_with_infissi(
        db: State<'_, Database>,
        stanza: StanzaDto,
    ) -> Result<StanzaDto, String> {
        todo!()
    }
}
