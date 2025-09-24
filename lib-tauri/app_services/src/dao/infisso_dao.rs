use crate::dao::utils::{map_error_for_entity, EntityType};
use app_models::models::{Infisso, NewInfisso, UpdateInfisso};
use app_models::schema::infisso;
use app_utils::app_error::DomainError;
use app_utils::app_interface::dao_interface::crud_operations::{Get, GetAll, Insert, Update};
use app_utils::app_interface::dao_interface::DAO;
use app_utils::app_interface::database_interface::PostgresPooled;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

pub struct InfissoDAO;

impl DAO for InfissoDAO {}

impl GetAll<Infisso> for InfissoDAO {
    type Output = Infisso;
    fn get_all(conn: &mut PostgresPooled) -> Result<Vec<Self::Output>, DomainError> {
        infisso::table
            .load(conn)
            .map_err(|e| map_error_for_entity(e, EntityType::Infisso))
    }
}

impl Get<Infisso, String> for InfissoDAO {
    type Output = Vec<Infisso>;

    fn get(conn: &mut PostgresPooled, id_edificio: String) -> Result<Self::Output, DomainError> {
        infisso::table
            .filter(infisso::edificio_id.eq(&id_edificio))
            .load(conn)
            .map_err(|e| map_error_for_entity(e, EntityType::Infisso))
    }
}

impl Insert<NewInfisso> for InfissoDAO {
    type Output = Infisso;
    fn insert(conn: &mut PostgresPooled, item: NewInfisso) -> Result<Self::Output, DomainError> {
        diesel::insert_into(infisso::table)
            .values(&item)
            .get_result(conn)
            .map_err(|e| map_error_for_entity(e, EntityType::Infisso))
    }
}

impl Update<UpdateInfisso, (String, String)> for InfissoDAO {
    type Output = Infisso;

    /// id -> (infisso, edificio)
    fn update(
        conn: &mut PostgresPooled,
        id: (String, String),
        item: UpdateInfisso,
    ) -> Result<Self::Output, DomainError> {
        let id_infisso = id.0;
        let id_edificio = id.1;

        diesel::update(infisso::table.find((id_infisso, id_edificio)))
            .set(&item)
            .get_result(conn)
            .map_err(|e| map_error_for_entity(e, EntityType::Infisso))
    }
}
