use app_models::models::TipoInfisso;
use app_models::schema::tipo_infisso;
use app_utils::app_error::DomainError;
use app_utils::app_interface::dao_interface::crud_operations::{GetAll, Insert};
use app_utils::app_interface::dao_interface::DAO;
use app_utils::app_interface::database_interface::PostgresPooled;
use diesel::RunQueryDsl;

pub struct TipoInfissoDAO;

impl DAO for TipoInfissoDAO {}

impl GetAll<TipoInfisso> for TipoInfissoDAO {
    type Output = TipoInfisso;

    fn get_all(conn: &mut PostgresPooled) -> Result<Vec<Self::Output>, DomainError> {
        tipo_infisso::table.load(conn).map_err(DomainError::from)
    }
}

/*
impl Insert<TipoInfisso> for TipoInfissoDAO {
    fn insert<C: DatabaseConnection>(conn: &C, item: TipoInfisso) -> Result<TipoInfisso, AppError> {
        let query = format!(
            "INSERT OR IGNORE INTO {}(NOME) VALUES (?1) RETURNING ID",
            Self::table_name()
        );
        let mut stmt = conn.prepare(query.as_str())?;
        let mut res = stmt.query_map(params![item.nome], |row| row.get::<_, u64>(0))?;
        let id = res.next().unwrap()?;
        info!("TipoInfisso inserito con ID {}", item.nome);
        Ok(TipoInfisso { _id: id, ..item })
    }
}
*/
