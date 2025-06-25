use crate::app_traits::{CreateTable, SqlExecutor};
use crate::dao::dati_stanze_view_dao::DatiStanzeViewDAO;
use crate::dao::tipo_infisso_dao::TipoInfissoDAO;
use crate::dao::{
    AnnotazioneEdificioDAO, AnnotazioneInfissoDAO, AnnotazioneStanzaDAO, ClimatizzazioneDAO,
    EdificioDAO, FotovoltaicoDAO, IlluminazioneDAO, InfissoDAO, MaterialeInfissoDAO,
    StanzaConInfissiDao, StanzaDAO, UtenzeDAO, VetroInfissoDAO,
};
use crate::utils::AppError;

pub fn create_tables<C: SqlExecutor>(conn: &C) -> Result<(), AppError> {
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

pub fn create_types_tables<C: SqlExecutor>(conn: &C) -> Result<(), AppError> {
    TipoInfissoDAO::create_table(conn)?;
    MaterialeInfissoDAO::create_table(conn)?;
    VetroInfissoDAO::create_table(conn)?;
    IlluminazioneDAO::create_table(conn)?;
    ClimatizzazioneDAO::create_table(conn)?;
    Ok(())
}
