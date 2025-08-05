use crate::dao::crud_operations::GetAll;
use crate::dao::entity::DatiStanza;
use crate::dao::mat_min_eff_stanza_view_dao::MatMinEffStanzaViewDao;
use crate::dao::mq_infissi_view_dao::MqInfissiViewDAO;
use crate::dao::schema_operations::CreateView;
use crate::dao::utils::DAO;
use crate::database::{DatabaseConnection, QueryBuilder, SqlQueryBuilder};
use crate::utils::AppError;
use rusqlite::Error;
use crate::dao::vet_min_eff_stanza_view_dao::VetMinEffStanzaViewDao;

pub struct DatiStanzeViewDAO;

impl DAO for DatiStanzeViewDAO {
    fn table_name() -> &'static str {
        "V_DATI_STANZE"
    }
}

impl CreateView for DatiStanzeViewDAO {
    fn create_view<C: DatabaseConnection>(conn: &C) -> Result<(), AppError> {
        conn.execute(
            format!(
                "CREATE VIEW IF NOT EXISTS {} AS
                SELECT S.ID, E.FASCICOLO, S.CHIAVE, S.PIANO, S.ID_SPAZIO, 
                    S.STANZA, S.DESTINAZIONE_USO,
                    S.ALTEZZA, S.SPESSORE_MURO,
                    S.RISCALDAMENTO, S.RAFFRESCAMENTO, S.ILLUMINAZIONE, 
                    coalesce(round(DGS.MQ_INFISSI, 2), 0) AS MQ_INFISSI,
                    DGS.MATERIALE, DGS.VETRO
                FROM STANZA AS S
                     JOIN EDIFICIO AS E ON E.CHIAVE = S.CHIAVE
                     -- Aggiungo i dati degli infissi alle stanze che c'è li hanno
                     LEFT JOIN (
                       -- Recupero i dati delle stanze che hanno degli infissi
                       SELECT MQ.ID_STANZA, MQ.MQ_INFISSI, M.MATERIALE, V.VETRO
                       FROM {} AS MQ
                                JOIN {} AS M 
                                    ON MQ.ID_STANZA = M.ID_STANZA
                                JOIN {} AS V 
                                    ON M.ID_STANZA = v.ID_STANZA
                       ) AS DGS ON S.ID = DGS.ID_STANZA;",
                Self::table_name(),
                MqInfissiViewDAO::table_name(),
                MatMinEffStanzaViewDao::table_name(),
                VetMinEffStanzaViewDao::table_name(),
            )
            .as_str(),
            (),
        )?;
        Ok(())
    }
}

impl GetAll<DatiStanza> for DatiStanzeViewDAO {
    fn get_all<C: DatabaseConnection>(conn: &C) -> Result<Vec<DatiStanza>, AppError> {
        let (query, _) = QueryBuilder::select().table(Self::table_name()).build()?;

        let mut stmt = conn.prepare(query.as_str())?;
        let dati_stanze: Result<Vec<DatiStanza>, Error> = stmt
            .query_map([], |row| {
                Ok(DatiStanza {
                    id: row.get("ID")?,
                    fascicolo: row.get("FASCICOLO")?,
                    chiave: row.get("CHIAVE")?,
                    piano: row.get("PIANO")?,
                    id_spazio: row.get("ID_SPAZIO")?,
                    stanza: row.get("STANZA")?,
                    destinazione_uso: row.get("DESTINAZIONE_USO")?,
                    altezza: row.get("ALTEZZA")?,
                    spessore_muro: row.get("SPESSORE_MURO")?,
                    riscaldamento: row.get("RISCALDAMENTO")?,
                    raffrescamento: row.get("RAFFRESCAMENTO")?,
                    illuminazione: row.get("ILLUMINAZIONE")?,
                    mq_infissi: row.get("MQ_INFISSI")?,
                    materiale: row.get("MATERIALE")?,
                    vetro: row.get("VETRO")?,
                })
            })?
            .collect();
        dati_stanze.map_err(AppError::from)
    }
}
