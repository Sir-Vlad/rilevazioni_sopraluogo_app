use crate::dao::{StanzaConInfissiDao, StanzaDAO};
use crate::dto::StanzaDTO;
use app_error::AppResult;
use app_interface::dao_interface::crud_operations::{Get, GetAll, Insert, Update};
use app_interface::database_interface::DatabaseManager;
use app_interface::service_interface::{CreateService, RetrieveManyService, UpdateService};
use app_models::models::StanzaConInfissi;
use app_models::schema::stanza::dsl::stanza;
use async_trait::async_trait;
use diesel::Connection;
use tauri::State;

pub struct StanzaService;

#[async_trait]
impl RetrieveManyService<StanzaDTO> for StanzaService {
    async fn retrieve_many(
        db: State<'_, impl DatabaseManager + Send + Sync>,
    ) -> AppResult<Vec<StanzaDTO>> {
        let mut conn = db.get_connection().await?;

        conn.transaction::<_, _, _>(|conn| {
            let stanze = StanzaDAO::get_all(conn)?;
            let mut stanze_dto: Vec<StanzaDTO> = stanze.iter().map(StanzaDTO::from).collect();

            let infissi = StanzaConInfissiDao::get(conn, ())?;

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
        })


    }
}

#[async_trait]
impl CreateService<StanzaDTO> for StanzaService {
    async fn create(
        db: State<'_, impl DatabaseManager + Send + Sync>,
        item: StanzaDTO,
    ) -> AppResult<StanzaDTO> {
        let conn = db.get_conn();
        if let Some(conn) = conn.as_ref() {
            let result = StanzaDAO::insert(conn, stanza.clone().into())?;
            Ok(StanzaDTO::from(result))
        } else {
            Err(AppError::DatabaseNotInitialized)
        }
    }
}

#[async_trait]
impl UpdateService<StanzaDTO> for StanzaService {
    async fn update(
        db: State<'_, impl DatabaseManager + Send + Sync>,
        item: StanzaDTO,
    ) -> AppResult<StanzaDTO> {
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
