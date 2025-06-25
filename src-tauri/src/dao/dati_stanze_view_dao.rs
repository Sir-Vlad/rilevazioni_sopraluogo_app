use crate::app_traits::{CreateTable, CreateView, DaoTrait, EntityTrait, GetAll, SqlExecutor};
use crate::entities::{DatiStanza, MatMinEffStanza, MqInfissi, VetMinEffStanza};
use crate::utils::AppError;

pub struct DatiStanzeViewDAO;

impl DaoTrait for DatiStanzeViewDAO {
    type Entity = DatiStanza;
    type Error = AppError;
}

impl CreateTable for DatiStanzeViewDAO {
    fn create_table<Connection: SqlExecutor>(conn: &Connection) -> Result<(), Self::Error> {
        conn.execute(&Self::Entity::sql_create_table(), &[])?;
        MatMinEffStanzaViewDao::create_table(conn)?;
        VetMinEffStanzaViewDao::create_table(conn)?;
        MqInfissiViewDao::create_table(conn)?;
        Ok(())
    }
}

impl CreateView for DatiStanzeViewDAO {}

impl GetAll for DatiStanzeViewDAO {}

pub struct MatMinEffStanzaViewDao;

impl DaoTrait for MatMinEffStanzaViewDao {
    type Entity = MatMinEffStanza;
    type Error = AppError;
}

impl CreateTable for MatMinEffStanzaViewDao {}

pub struct VetMinEffStanzaViewDao;

impl DaoTrait for VetMinEffStanzaViewDao {
    type Entity = VetMinEffStanza;
    type Error = AppError;
}

impl CreateTable for VetMinEffStanzaViewDao {}

pub struct MqInfissiViewDao;

impl DaoTrait for MqInfissiViewDao {
    type Entity = MqInfissi;
    type Error = AppError;
}

impl CreateTable for MqInfissiViewDao {}
