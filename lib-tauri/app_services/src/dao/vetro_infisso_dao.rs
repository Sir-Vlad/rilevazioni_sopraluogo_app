use app_utils::app_error::DomainError;
use app_utils::app_interface::dao_interface::crud_operations::{GetAll, Insert};
use app_utils::app_interface::dao_interface::DAO;
use app_utils::app_interface::database_interface::PostgresPooled;
use app_models::models::VetroInfisso;
use app_models::schema::vetro_infisso;
use diesel::RunQueryDsl;

pub struct VetroInfissoDAO;

impl DAO for VetroInfissoDAO {}

impl GetAll<VetroInfisso> for VetroInfissoDAO {
    type Output = VetroInfisso;
    fn get_all(conn: &mut PostgresPooled) -> Result<Vec<Self::Output>, DomainError> {
        vetro_infisso::table.load(conn).map_err(DomainError::from)
    }
}
/*
impl Insert<VetroInfisso> for VetroInfissoDAO {
    fn insert<C: DatabaseConnection>(
        conn: &C,
        item: VetroInfisso,
    ) -> Result<VetroInfisso, AppError> {
        let builder = QueryBuilder::insert()
            .table(Self::table_name())
            .columns(vec!["VETRO", "EFFICIENZA_ENERGETICA"])
            .values(vec![
                item.vetro.clone().into(),
                item.efficienza_energetica.into(),
            ])
            .returning("ID");
        let (query, param) = builder.build()?;
        let mut stmt = conn.prepare(query.as_str())?;
        let mut res = stmt.query_map(rusqlite::params_from_iter(convert_param(param)), |row| {
            row.get::<_, u64>(0)
        })?;
        let id = res.next().unwrap()?;
        info!("VetroInfisso inserito con ID {}", item.vetro);
        Ok(VetroInfisso {
            _id: Some(id),
            ..item
        })
    }
}

 */
