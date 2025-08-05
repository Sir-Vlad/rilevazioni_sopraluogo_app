use crate::dao::schema_operations::CreateView;
use crate::dao::utils::DAO;
use crate::database::DatabaseConnection;
use crate::utils::AppError;

pub struct MatMinEffStanzaViewDao;

impl DAO for MatMinEffStanzaViewDao {
    fn table_name() -> &'static str {
        "V_MAT_MIN_EFF_STANZA"
    }
}

impl CreateView for MatMinEffStanzaViewDao {
    fn create_view<C: DatabaseConnection>(conn: &C) -> Result<(), AppError> {
        conn.execute(
            format!(
                "CREATE VIEW IF NOT EXISTS {} AS
                SELECT SI.ID_STANZA, MI.MATERIALE
                FROM STANZA AS S
                     JOIN STANZA_CON_INFISSI AS SI ON S.ID = SI.ID_STANZA
                     JOIN INFISSO AS I ON SI.ID_INFISSO = I.ID
                     JOIN MATERIALE_INFISSO AS MI ON I.MATERIALE = MI.MATERIALE
                WHERE MI.EFFICIENZA_ENERGETICA IN (
                      SELECT MIN(MI2.EFFICIENZA_ENERGETICA)
                      FROM STANZA_CON_INFISSI AS SI
                               JOIN INFISSO AS I2 ON SI.ID_INFISSO = I2.ID
                               JOIN MATERIALE_INFISSO AS MI2 ON I2.MATERIALE = MI2.MATERIALE
                      WHERE SI.ID_STANZA = S.ID)
                GROUP BY SI.ID_STANZA, MI.MATERIALE;",
                Self::table_name()
            )
            .as_str(),
            (),
        )?;
        Ok(())
    }
}
