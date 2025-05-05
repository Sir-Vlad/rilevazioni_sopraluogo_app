use crate::dao::schema_operations::CreateView;
use crate::dao::utils::DAO;
use crate::database::DatabaseConnection;
use crate::utils::AppError;

pub struct MqInfissiViewDAO;

impl DAO for MqInfissiViewDAO {
    fn table_name() -> &'static str {
        "V_MQ_INFISSI"
    }
}

impl CreateView for MqInfissiViewDAO {
    fn create_view<C: DatabaseConnection>(conn: &C) -> Result<(), AppError> {
        conn.execute(
            format!(
                "CREATE VIEW IF NOT EXISTS {} AS
                SELECT SI.ID_STANZA, SUM(I.MQ * SI.NUM_INFISSI) AS 'MQ_INFISSI'
                FROM INFISSO AS I
                         JOIN STANZA_CON_INFISSI AS SI ON I.ID = SI.ID_INFISSO
                GROUP BY SI.ID_STANZA;",
                Self::table_name()
            )
            .as_str(),
            (),
        )?;
        Ok(())
    }
}
