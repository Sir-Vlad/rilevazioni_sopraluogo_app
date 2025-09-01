use app_models::models::Illuminazione;
use app_models::schema::illuminazione;
use app_utils::app_error::DomainError;
use app_utils::app_interface::dao_interface::crud_operations::{GetAll, Insert};
use app_utils::app_interface::dao_interface::DAO;
use app_utils::app_interface::database_interface::PostgresPooled;
use diesel::result::Error;
use diesel::RunQueryDsl;

pub struct IlluminazioneDAO;

impl DAO for IlluminazioneDAO {}

impl GetAll<Illuminazione> for IlluminazioneDAO {
    type Output = Illuminazione;
    fn get_all(conn: &mut PostgresPooled) -> Result<Vec<Self::Output>, DomainError> {
        illuminazione::table
            .load::<Illuminazione>(conn)
            .map_err(|e| match e {
                Error::NotFound => DomainError::IlluminazioneNotFound,
                _ => DomainError::Unexpected(e),
            })
    }
}

/*
impl Insert<NewIlluminazione> for IlluminazioneDAO {
    fn insert<C: DatabaseConnection>(
        conn: &C,
        item: Illuminazione,
    ) -> Result<Illuminazione, AppError> {
        let builder = QueryBuilder::insert()
            .table(Self::table_name())
            .columns(vec!["LAMPADINA", "EFFICIENZA_ENERGETICA"])
            .values(vec![
                item.lampadina.clone().into(),
                item.efficienza_energetica.into(),
            ])
            .returning("ID");
        let (query, param) = builder.build()?;
        let mut stmt = conn.prepare(query.as_str())?;
        let mut res = stmt.query_map(rusqlite::params_from_iter(convert_param(param)), |row| {
            row.get::<_, u64>(0)
        })?;
        let id = res.next().unwrap()?;
        info!("Illuminazione inserita con ID {}", item.lampadina);
        Ok(Illuminazione {
            _id: Some(id),
            ..item
        })
    }
}
 */
