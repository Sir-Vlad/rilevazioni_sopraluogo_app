use crate::dao::dati_stanze_view_dao::DatiStanzeViewDAO;
use crate::dao::mat_min_eff_stanza_view_dao::MatMinEffStanzaViewDao;
use crate::dao::mq_infissi_view_dao::MqInfissiViewDAO;
use crate::dao::schema_operations::{CreateTable, CreateView};
use crate::dao::tipo_infisso_dao::TipoInfissoDAO;
use crate::dao::vet_min_eff_stanza_view_dao::VetMinEffStanzaViewDao;
use crate::dao::{
    AnnotazioneEdificioDAO, AnnotazioneInfissoDAO, AnnotazioneStanzaDAO, ClimatizzazioneDAO,
    EdificioDAO, FotovoltaicoDAO, IlluminazioneDAO, InfissoDAO, MaterialeInfissoDAO,
    StanzaConInfissiDao, StanzaDAO, UtenzeDAO, VetroInfissoDAO,
};
use crate::database::DatabaseConnection;
use crate::utils::AppError;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};

pub trait DAO {
    fn table_name() -> &'static str;
}

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

pub fn create_tables<C: DatabaseConnection>(conn: &C) -> Result<(), AppError> {
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
    Ok(())
}

pub fn create_types_tables<C: DatabaseConnection>(conn: &C) -> Result<(), AppError> {
    TipoInfissoDAO::create_table(conn)?;
    MaterialeInfissoDAO::create_table(conn)?;
    VetroInfissoDAO::create_table(conn)?;
    IlluminazioneDAO::create_table(conn)?;
    ClimatizzazioneDAO::create_table(conn)?;
    Ok(())
}

pub fn create_views<C: DatabaseConnection>(conn: &C) -> Result<(), AppError> {
    MqInfissiViewDAO::create_view(conn)?;
    VetMinEffStanzaViewDao::create_view(conn)?;
    MatMinEffStanzaViewDao::create_view(conn)?;
    DatiStanzeViewDAO::create_view(conn)?;
    Ok(())
}

pub(crate) fn convert_timestamp_to_local(timestamp: String) -> Result<String, AppError> {
    let naive_dt = NaiveDateTime::parse_from_str(&timestamp, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| AppError::GenericError(e.to_string()))?;
    let local_time: DateTime<Local> = DateTime::from(Utc.from_utc_datetime(&naive_dt));
    Ok(local_time.format("%Y-%m-%d %H:%M:%S").to_string())
}
