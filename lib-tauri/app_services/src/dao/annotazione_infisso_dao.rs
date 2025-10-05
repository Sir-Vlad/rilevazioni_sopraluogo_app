use app_models::{
    models::{AnnotazioneInfisso, NewAnnotazioneInfisso},
    schema::annotazione_infisso,
};
use app_utils::{
    app_error::DomainError,
    app_interface::{
        dao_interface::{
            DAO,
            crud_operations::{GetAll, Insert},
        },
        database_interface::PostgresPooled,
    },
};
use diesel::RunQueryDsl;

use crate::dao::utils::map_error_annotazione;

pub struct AnnotazioneInfissoDAO;

impl DAO for AnnotazioneInfissoDAO {}

impl GetAll<AnnotazioneInfisso> for AnnotazioneInfissoDAO {
    type Output = AnnotazioneInfisso;

    fn get_all(conn: &mut PostgresPooled) -> Result<Vec<Self::Output>, DomainError> {
        annotazione_infisso::table
            .load(conn)
            .map_err(DomainError::from)
    }
}

impl Insert<NewAnnotazioneInfisso> for AnnotazioneInfissoDAO {
    type Output = AnnotazioneInfisso;

    fn insert(
        conn: &mut PostgresPooled,
        item: NewAnnotazioneInfisso,
    ) -> Result<Self::Output, DomainError> {
        diesel::insert_into(annotazione_infisso::table)
            .values(&item)
            .get_result(conn)
            .map_err(map_error_annotazione)
    }
}
