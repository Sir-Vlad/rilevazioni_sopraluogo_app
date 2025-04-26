use crate::dao::{StanzaDAO, StanzaDAOImpl};
use crate::database::Database;
use crate::dto::StanzaDTO;
use tauri::State;
pub trait StanzaService {
    fn get_all(db: State<'_, Database>) -> Result<Vec<StanzaDTO>, String>;
    fn insert(db: State<'_, Database>, stanza: StanzaDTO) -> Result<StanzaDTO, String>;
    fn update(db: State<'_, Database>, stanza: StanzaDTO) -> Result<StanzaDTO, String>;
    fn get_with_infissi(db: State<'_, Database>, id: i64) -> Result<Vec<StanzaDTO>, String>;
    fn insert_with_infissi(db: State<'_, Database>, stanza: StanzaDTO)
        -> Result<StanzaDTO, String>;
}

pub struct StanzaServiceImpl;

impl StanzaService for StanzaServiceImpl {
    fn get_all(db: State<'_, Database>) -> Result<Vec<StanzaDTO>, String> {
        let conn = db.get_conn();
        if let Some(conn) = conn.as_ref() {
            let stanze = StanzaDAOImpl::get_all(conn)?;
            let mut stanze_dto: Vec<StanzaDTO> = stanze.iter().map(StanzaDTO::from).collect();
            let infissi = StanzaDAOImpl::get_infissi_by_all(conn)?;

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

    fn insert(db: State<'_, Database>, stanza: StanzaDTO) -> Result<StanzaDTO, String> {
        let conn = db.get_conn();
        if let Some(conn) = conn.as_ref() {
            let result = StanzaDAOImpl::insert(conn, stanza.clone().into())?;
            Ok(StanzaDTO::from(result))
        } else {
            Err("Database not initialized".to_string())
        }
    }

    fn update(db: State<'_, Database>, stanza: StanzaDTO) -> Result<StanzaDTO, String> {
        let conn = db.get_conn();
        // fixme: convertire in transazionale
        if let Some(conn) = conn.as_ref() {
            let result = StanzaDAOImpl::update(conn, stanza.clone().into())?;
            if let Some(infissi) = &stanza.infissi {
                StanzaDAOImpl::set_infissi_by_id(conn, stanza.id, infissi.clone())?;
            }
            Ok(StanzaDTO::from(result))
        } else {
            Err("Database not initialized".to_string())
        }
    }

    #[allow(dead_code, unused_variables)]
    fn get_with_infissi(db: State<'_, Database>, id: i64) -> Result<Vec<StanzaDTO>, String> {
        todo!()
    }
    #[allow(dead_code, unused_variables)]
    fn insert_with_infissi(
        db: State<'_, Database>,
        stanza: StanzaDTO,
    ) -> Result<StanzaDTO, String> {
        todo!()
    }
}
