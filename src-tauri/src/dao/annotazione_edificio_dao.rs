use crate::dao::crud_operations::{GetAll, Insert};
use crate::dao::entity::AnnotazioneEdificio;
use crate::dao::utils::schema_operations::CreateTable;
use crate::dao::utils::DAO;
use crate::database::DatabaseConnection;
use crate::utils::AppError;

pub struct AnnotazioneEdificioDAO;

impl DAO for AnnotazioneEdificioDAO {
    fn table_name() -> &'static str {
        "ANNOTAZIONE_EDIFICIO"
    }
}

impl CreateTable for AnnotazioneEdificioDAO {
    fn create_table<C: DatabaseConnection>(conn: &C) -> Result<(), AppError> {
        conn.execute(
            format!(
                "CREATE TABLE IF NOT EXISTS {}
                (
                    ID          INTEGER PRIMARY KEY AUTOINCREMENT,
                    ID_EDIFICIO TEXT REFERENCES EDIFICIO (CHIAVE),
                    CONTENT     TEXT NOT NULL CHECK ( length(CONTENT) > 0 ),
                    DATA        TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
                ) STRICT;",
                Self::table_name()
            )
            .as_str(),
            (),
        )?;
        Ok(())
    }
}

impl GetAll<AnnotazioneEdificio> for AnnotazioneEdificioDAO {
    fn get_all<C: DatabaseConnection>(conn: &C) -> Result<Vec<AnnotazioneEdificio>, AppError> {
        todo!()
    }
}

impl Insert<AnnotazioneEdificio> for AnnotazioneEdificioDAO {
    fn insert<C: DatabaseConnection>(
        conn: &C,
        item: AnnotazioneEdificio,
    ) -> Result<AnnotazioneEdificio, AppError> {
        todo!()
    }
}
