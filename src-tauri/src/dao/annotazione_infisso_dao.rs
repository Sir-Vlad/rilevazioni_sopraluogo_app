use crate::dao::crud_operations::{GetAll, Insert};
use crate::dao::entity::AnnotazioneInfisso;
use crate::dao::utils::schema_operations::CreateTable;
use crate::dao::utils::DAO;
use crate::database::DatabaseConnection;
use crate::utils::AppError;

pub struct AnnotazioneInfissoDAO;

impl DAO for AnnotazioneInfissoDAO {
    fn table_name() -> &'static str {
        "ANNOTAZIONE_INFISSO"
    }
}

impl CreateTable for AnnotazioneInfissoDAO {
    fn create_table<C: DatabaseConnection>(conn: &C) -> Result<(), AppError> {
        conn.execute(
            format!(
                "CREATE TABLE IF NOT EXISTS {}
                (
                    ID         INTEGER PRIMARY KEY AUTOINCREMENT,
                    ID_INFISSO TEXT NOT NULL REFERENCES INFISSO (ID),
                    CONTENT    TEXT NOT NULL CHECK ( length(CONTENT) > 0 ),
                    DATA       TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
                ) STRICT;",
                Self::table_name()
            )
            .as_str(),
            (),
        )?;
        Ok(())
    }
}

impl GetAll<AnnotazioneInfisso> for AnnotazioneInfissoDAO {
    fn get_all<C: DatabaseConnection>(conn: &C) -> Result<Vec<AnnotazioneInfisso>, AppError> {
        todo!()
    }
}

impl Insert<AnnotazioneInfisso> for AnnotazioneInfissoDAO {
    fn insert<C: DatabaseConnection>(
        conn: &C,
        item: AnnotazioneInfisso,
    ) -> Result<AnnotazioneInfisso, AppError> {
        todo!()
    }
}
