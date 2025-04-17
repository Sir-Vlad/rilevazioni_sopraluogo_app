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
            let stanze = StanzaDaoImpl::get_all(conn)?;
            let mut stanze_dto: Vec<StanzaDto> = stanze.iter().map(StanzaDto::from).collect();
            let infissi = StanzaDaoImpl::get_infissi_by_all(conn)?;

            for stanza in &mut stanze_dto {
                let infisso = infissi.get(&stanza.id.to_string());
                if let Some(infisso) = infisso {
                    stanza.infissi = Some(infisso.clone());
                } else {
                    continue;
                }
            }

            Ok(stanze_dto)
        } else {
            Err("Database not initialized".to_string())
        }
    }

    fn insert(db: State<'_, Database>, stanza: StanzaDto) -> Result<StanzaDto, String> {
        let conn = db.get_conn();
        if let Some(conn) = conn.as_ref() {
            let result = StanzaDaoImpl::insert(conn, stanza.clone().into())?;
            Ok(StanzaDto::from(result))
        } else {
            Err("Database not initialized".to_string())
        }
    }

    fn update(db: State<'_, Database>, stanza: StanzaDto) -> Result<StanzaDto, String> {
        let conn = db.get_conn();
        // fixme: convertire in transazionale
        if let Some(conn) = conn.as_ref() {
            let result = StanzaDaoImpl::update(conn, stanza.clone().into())?;
            if let Some(infissi) = &stanza.infissi {
                StanzaDaoImpl::set_infissi_by_id(conn, stanza.id, infissi.clone())?;
            }
            Ok(StanzaDto::from(result))
        } else {
            Err("Database not initialized".to_string())
        }
    }

    #[allow(dead_code, unused_variables)]
    fn get_with_infissi(db: State<'_, Database>, id: i64) -> Result<Vec<StanzaDto>, String> {
        todo!()
    }
    #[allow(dead_code, unused_variables)]
    fn insert_with_infissi(
        db: State<'_, Database>,
        stanza: StanzaDto,
    ) -> Result<StanzaDto, String> {
        todo!()
    }
}
