use crate::app_traits::{CreateTable as CreateTableNew, SqlExecutor};
use crate::dao::dati_stanze_view_dao::DatiStanzeViewDAO;
use crate::dao::schema_operations::{CreateTable, CreateView};
use crate::dao::tipo_infisso_dao::TipoInfissoDAO;
use crate::dao::{
    AnnotazioneEdificioDAO, AnnotazioneInfissoDAO, AnnotazioneStanzaDAO, ClimatizzazioneDAO,
    EdificioDAO, FotovoltaicoDAO, IlluminazioneDAO, InfissoDAO, MaterialeInfissoDAO,
    StanzaConInfissiDao, StanzaDAO, UtenzeDAO, VetroInfissoDAO,
};
use crate::database::DatabaseConnection;
use crate::utils::AppError;
use chrono::TimeZone;

pub trait DAO {
    fn table_name() -> &'static str;
}

#[deprecated]
pub mod schema_operations {
    use crate::dao::utils::DAO;
    use crate::database::DatabaseConnection;
    use crate::utils::AppError;

    pub trait CreateTable: DAO {
        fn create_table<C: DatabaseConnection>(conn: &C) -> Result<(), AppError>;
    }

    pub trait CreateView: DAO {
        fn create_view<C: DatabaseConnection>(conn: &C) -> Result<(), AppError>;
    }
}

#[deprecated]
pub mod crud_operations {
    use crate::dao::utils::DAO;
    use crate::database::DatabaseConnection;
    use crate::utils::AppError;

    pub trait Get<T, K>: DAO {
        fn get<C: DatabaseConnection>(conn: &C, id: K) -> Result<T, AppError>;
    }

    pub trait GetAll<T>: DAO {
        fn get_all<C: DatabaseConnection>(conn: &C) -> Result<Vec<T>, AppError>;
    }

    pub trait Insert<T>: DAO {
        fn insert<C: DatabaseConnection>(conn: &C, item: T) -> Result<T, AppError>;
    }

    pub trait Update<T>: DAO {
        fn update<C: DatabaseConnection>(conn: &C, item: T) -> Result<T, AppError>;
    }

    #[allow(dead_code)]
    pub trait Delete<T, K>: DAO {
        fn delete<C: DatabaseConnection>(conn: &C, item: K) -> Result<bool, AppError>;
    }
}

pub fn create_tables<C: DatabaseConnection + SqlExecutor>(conn: &C) -> Result<(), AppError> {
    create_types_tables(conn)?;

    InfissoDAO::create_table(conn)?;
    EdificioDAO::create_table(conn)?;
    StanzaDAO::create_table(conn)?;
    StanzaConInfissiDao::create_table(conn)?;

    AnnotazioneStanzaDAO::create_table(conn)?;
    AnnotazioneEdificioDAO::create_table(conn)?;
    AnnotazioneInfissoDAO::create_table(conn)?;

    FotovoltaicoDAO::create_table(conn)?;
    UtenzeDAO::create_table(conn)?;

    DatiStanzeViewDAO::create_table(conn)?;
    Ok(())
}

pub fn create_types_tables<C: DatabaseConnection + SqlExecutor>(conn: &C) -> Result<(), AppError> {
    TipoInfissoDAO::create_table(conn)?;
    MaterialeInfissoDAO::create_table(conn)?;
    VetroInfissoDAO::create_table(conn)?;
    IlluminazioneDAO::create_table(conn)?;
    ClimatizzazioneDAO::create_table(conn)?;
    Ok(())
}
