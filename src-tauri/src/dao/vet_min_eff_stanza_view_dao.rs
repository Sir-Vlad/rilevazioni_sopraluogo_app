use crate::dao::schema_operations::CreateView;
use crate::dao::utils::DAO;
use crate::database::DatabaseConnection;
use crate::utils::AppError;

pub struct VetMinEffStanzaViewDao;

impl DAO for VetMinEffStanzaViewDao {
    fn table_name() -> &'static str {
        "V_VET_MIN_EFF_STANZA"
    }
}

impl CreateView for VetMinEffStanzaViewDao {
    fn create_view<C: DatabaseConnection>(conn: &C) -> Result<(), AppError> {
        conn.execute(
            format!(
                "CREATE VIEW IF NOT EXISTS {} AS
                SELECT SI.ID_STANZA, MI.VETRO
                FROM STANZA AS S
                         JOIN STANZA_CON_INFISSI AS SI ON S.ID = SI.ID_STANZA
                         JOIN INFISSO AS I ON SI.ID_INFISSO = I.ID
                         JOIN VETRO_INFISSO AS MI ON I.VETRO = MI.VETRO
                WHERE MI.EFFICIENZA_ENERGETICA IN (
                          SELECT MIN(MI2.EFFICIENZA_ENERGETICA)
                          FROM STANZA_CON_INFISSI AS SI
                                   JOIN INFISSO AS I2 ON SI.ID_INFISSO = I2.ID
                                   JOIN VETRO_INFISSO AS MI2 ON I2.VETRO = MI2.VETRO
                          WHERE SI.ID_STANZA = S.ID)
                GROUP BY SI.ID_STANZA, MI.VETRO;",
                Self::table_name()
            )
            .as_str(),
            (),
        )?;
        Ok(())
    }
}
