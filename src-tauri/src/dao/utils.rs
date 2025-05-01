use crate::dao::schema_operations::CreateTable;
use crate::dao::{
    AnnotazioneEdificioDAO, AnnotazioneInfissoDAO, AnnotazioneStanzaDAO, ClimatizzazioneDAO,
    EdificioDAO, FotovoltaicoDAO, IlluminazioneDAO, InfissoDAO, MaterialeInfissoDAO, StanzaDAOImpl,
    UtenzeDAO, VetroInfissoDAO,
};
use crate::database::DatabaseConnection;

pub trait DAO {
    fn table_name() -> &'static str;
}

pub mod schema_operations {
    use crate::dao::utils::DAO;
    use crate::database::DatabaseConnection;
    pub trait CreateTable: DAO {
        fn create_table<C: DatabaseConnection>(conn: &C) -> Result<(), String>;
    }
}

pub mod crud_operations {
    use crate::dao::utils::DAO;
    use crate::database::DatabaseConnection;

    pub trait Get<T, K>: DAO {
        fn get<C: DatabaseConnection>(conn: &C, id: K) -> Result<T, String>;
    }

    pub trait GetAll<T>: DAO {
        fn get_all<C: DatabaseConnection>(conn: &C) -> Result<Vec<T>, String>;
    }

    pub trait Insert<T>: DAO {
        fn insert<C: DatabaseConnection>(conn: &C, item: T) -> Result<T, String>;
    }

    pub trait Update<T>: DAO {
        fn update<C: DatabaseConnection>(conn: &C, item: T) -> Result<T, String>;
    }

    pub trait Delete<T, K>: DAO {
        fn delete<C: DatabaseConnection>(conn: &C, item: K) -> Result<bool, String>;
    }
}
pub fn create_tables<C: DatabaseConnection>(conn: &C) -> Result<(), String> {
    MaterialeInfissoDAO::create_table(conn)?;
    VetroInfissoDAO::create_table(conn)?;
    IlluminazioneDAO::create_table(conn)?;
    ClimatizzazioneDAO::create_table(conn)?;

    InfissoDAO::create_table(conn)?;
    EdificioDAO::create_table(conn)?;
    StanzaDAOImpl::create_table(conn)?;

    AnnotazioneStanzaDAO::create_table(conn)?;
    AnnotazioneEdificioDAO::create_table(conn)?;
    AnnotazioneInfissoDAO::create_table(conn)?;

    FotovoltaicoDAO::create_table(conn)?;
    UtenzeDAO::create_table(conn)?;
    Ok(())
}
