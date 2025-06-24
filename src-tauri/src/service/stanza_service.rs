use crate::app_traits::{GetAll, Insert, Update};
use crate::dao::crud_operations::{GetAll as G, Update as U};
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
        let conn = db.get_conn()?;
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
        let conn = db.get_conn()?;
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
        db.with_transaction(|tx| {
            let stanza_aggiornata = StanzaDAO::update(tx, stanza.clone().into())?;
            let stanza_dto = match &stanza.infissi {
                Some(infissi) => {
                    let stanza_con_infissi = StanzaConInfissi::new_with_infissi_expanse(
                        stanza.id,
                        infissi.clone(),
                        stanza.chiave,
                    );

                    let infissi_aggiornati = StanzaConInfissiDao::update(tx, stanza_con_infissi)?;

                    let mut stanza_risultato = StanzaDTO::from(stanza_aggiornata);
                    stanza_risultato.infissi = Some(infissi_aggiornati.expanse_infissi());
                    stanza_risultato
                }
                None => StanzaDTO::from(stanza_aggiornata),
            };
            Ok(stanza_dto)
        })
    }
}
