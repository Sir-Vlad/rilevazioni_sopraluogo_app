use app_models::models::MaterialeInfisso;
use app_models::schema::materiale_infisso;
use app_utils::app_error::DomainError;
use app_utils::app_interface::dao_interface::crud_operations::GetAll;
use app_utils::app_interface::dao_interface::DAO;
use app_utils::app_interface::database_interface::PostgresPooled;
use diesel::RunQueryDsl;

pub struct MaterialeInfissoDAO;

impl DAO for MaterialeInfissoDAO {}

impl GetAll<MaterialeInfisso> for MaterialeInfissoDAO {
    type Output = MaterialeInfisso;
    fn get_all(conn: &mut PostgresPooled) -> Result<Vec<Self::Output>, DomainError> {
        materiale_infisso::table
            .load(conn)
            .map_err(|e| DomainError::from(e))
    }
}

/*
impl Insert<MaterialeInfisso> for MaterialeInfissoDAO {
    fn insert(
        conn: &mut PostgresPooled,
        item: MaterialeInfisso,
    ) -> Result<Self::Output, DomainError> {
        diesel::insert_into(materiale_infisso::table)
            .values(&item)
            .get_result(conn)
            .map_err(|e| match e {
                Error::NotFound => DomainError::MaterialeInfissoNotFound,
                Error::DatabaseError(kind, ref db_info) => {
                    if matches!(kind, diesel::result::DatabaseErrorKind::UniqueViolation) {
                        DomainError::MaterialeInfissoAlreadyExists
                    } else {
                        DomainError::from(e)
                    }
                }
                _ => DomainError::Unexpected(e),
            })
    }
}

 */
