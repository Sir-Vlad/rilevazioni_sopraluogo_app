use app_models::{models::Illuminazione, schema::illuminazione};
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
use diesel::{RunQueryDsl, result::Error};

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

impl Insert<Illuminazione> for IlluminazioneDAO {
    type Output = Illuminazione;

    fn insert(conn: &mut PostgresPooled, item: Illuminazione) -> Result<Self::Output, DomainError> {
        diesel::insert_into(illuminazione::table)
            .values(&item)
            .get_result(conn)
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
