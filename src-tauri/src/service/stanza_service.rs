use crate::dao::crud_operations::{GetAll, Insert, Update};
use crate::dao::entity::StanzaConInfissi;
use crate::dao::{StanzaConInfissiDao, StanzaDAO};
use crate::database::Database;
use crate::dto::StanzaDTO;
use crate::service::utils::{CreateService, RetrieveManyService, UpdateService};
use crate::utils::AppError;
use tauri::State;

pub struct StanzaService;

impl RetrieveManyService<StanzaDTO> for StanzaService {
    fn retrieve_many(db: State<'_, Database>) -> Result<Vec<StanzaDTO>, AppError> {
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
            Err(AppError::DatabaseNotInitialized)
        }
    }
}

impl CreateService<StanzaDTO> for StanzaService {
    fn create(db: State<'_, Database>, stanza: StanzaDTO) -> Result<StanzaDTO, AppError> {
        let conn = db.get_conn();
        if let Some(conn) = conn.as_ref() {
            let result = StanzaDAO::insert(conn, stanza.clone().into())?;
            Ok(StanzaDTO::from(result))
        } else {
            Err(AppError::DatabaseNotInitialized)
        }
    }
}

impl UpdateService<StanzaDTO> for StanzaService {
    fn update(db: State<'_, Database>, stanza: StanzaDTO) -> Result<StanzaDTO, AppError> {
        let res = db.with_transaction(|tx| {
            let result = StanzaDAO::update(tx, stanza.clone().into())?;
            if let Some(infissi) = &stanza.infissi {
                StanzaConInfissiDao::update(
                    tx,
                    StanzaConInfissi::new_with_infissi_expanse(
                        stanza.id,
                        infissi.clone(),
                        stanza.chiave,
                    ),
                )?;
            }
            Ok(StanzaDTO::from(result))
        })?;
        Ok(res)
    }
}
