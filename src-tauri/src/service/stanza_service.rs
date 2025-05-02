use crate::dao::crud_operations::{GetAll, Insert, Update};
use crate::dao::{StanzaConInfissiDao, StanzaDAO};
use crate::database::Database;
use crate::dto::StanzaDTO;
use tauri::State;
use crate::dao::entity::StanzaConInfissi;

pub trait StanzaService {
    fn get_all(db: State<'_, Database>) -> Result<Vec<StanzaDTO>, String>;
    fn insert(db: State<'_, Database>, stanza: StanzaDTO) -> Result<StanzaDTO, String>;
    fn update(db: State<'_, Database>, stanza: StanzaDTO) -> Result<StanzaDTO, String>;
}

pub struct StanzaServiceImpl;

impl StanzaService for StanzaServiceImpl {
    fn get_all(db: State<'_, Database>) -> Result<Vec<StanzaDTO>, String> {
        let conn = db.get_conn();
        if let Some(conn) = conn.as_ref() {
            let stanze = StanzaDAO::get_all(conn)?;
            let mut stanze_dto: Vec<StanzaDTO> = stanze.iter().map(StanzaDTO::from).collect();
            let infissi = StanzaConInfissiDao::get_all(conn)?;

            for stanza in &mut stanze_dto {
                let infissi = infissi.iter().find(|x| x.id_stanza == stanza.id);
                if let Some(infissi) = infissi {
                    let infissi: Vec<String> = infissi.expanse_infissi();
                    stanza.infissi = Some(infissi);
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
            let result = StanzaDAO::insert(conn, stanza.clone().into())?;
            Ok(StanzaDTO::from(result))
        } else {
            Err("Database not initialized".to_string())
        }
    }

    fn update(db: State<'_, Database>, stanza: StanzaDTO) -> Result<StanzaDTO, String> {
        let res = db.with_transaction(|tx| {
            let result = StanzaDAO::update(tx, stanza.clone().into())?;
            if let Some(infissi) = &stanza.infissi {
                StanzaConInfissiDao::update(tx, StanzaConInfissi::new_with_infissi_expanse(stanza.id, infissi.clone()))?;
            }
            Ok(StanzaDTO::from(result))
        })?;
        Ok(res)
    }
}
