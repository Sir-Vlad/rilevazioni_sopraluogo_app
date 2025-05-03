use crate::dao::crud_operations::{GetAll, Insert, Update};
use crate::dao::entity::Infisso;
use crate::dao::utils::schema_operations::CreateTable;
use crate::dao::utils::DAO;
use crate::database::{convert_param, DatabaseConnection, QueryBuilder, SqlQueryBuilder};
use crate::utils::AppError;
use log::{error, info};

pub struct InfissoDAO;

impl DAO for InfissoDAO {
    fn table_name() -> &'static str {
        "INFISSO"
    }
}

impl CreateTable for InfissoDAO {
    fn create_table<C: DatabaseConnection>(conn: &C) -> Result<(), AppError> {
        conn.execute(
            format!(
                "CREATE TABLE IF NOT EXISTS {}
                (
                    ID        TEXT PRIMARY KEY,
                    TIPO      TEXT    NOT NULL CHECK ( TIPO IN ('PORTA', 'FINESTRA') ) DEFAULT 'FINESTRA',
                    ALTEZZA   INTEGER NOT NULL CHECK ( ALTEZZA >= 0 ),
                    LARGHEZZA INTEGER NOT NULL CHECK ( LARGHEZZA >= 0 ),
                    MATERIALE TEXT    NOT NULL REFERENCES MATERIALE_INFISSO (MATERIALE),
                    VETRO     TEXT    NOT NULL REFERENCES VETRO_INFISSO (VETRO),
                    MQ        REAL GENERATED ALWAYS AS ((ALTEZZA * LARGHEZZA) / 10000.0) VIRTUAL,
                    UNIQUE (TIPO, ALTEZZA, LARGHEZZA, MATERIALE, VETRO)
                ) STRICT;", 
                Self::table_name()
            ).as_str(),
            ()
        )?;
        info!("Tabella INFISSO creata");
        Ok(())
    }
}

impl GetAll<Infisso> for InfissoDAO {
    fn get_all<C: DatabaseConnection>(conn: &C) -> Result<Vec<Infisso>, AppError> {
        let (query, _) = QueryBuilder::select().table(Self::table_name()).build()?;

        let mut stmt = conn.prepare(query.as_str())?;
        let infissi: Result<Vec<Infisso>, rusqlite::Error> = stmt
            .query_map([], |row| {
                Ok(Infisso {
                    id: row.get::<_, String>("ID")?,
                    tipo: row.get::<_, String>("TIPO")?,
                    altezza: row.get::<_, u16>("ALTEZZA")?,
                    larghezza: row.get::<_, u16>("LARGHEZZA")?,
                    materiale: row.get::<_, String>("MATERIALE")?,
                    vetro: row.get::<_, String>("VETRO")?,
                })
            })?
            .collect();

        match infissi {
            Ok(infissi) => Ok(infissi),
            Err(e) => Err(AppError::from(e)),
        }
    }
}

impl Insert<Infisso> for InfissoDAO {
    fn insert<C: DatabaseConnection>(conn: &C, item: Infisso) -> Result<Infisso, AppError> {
        let builder = QueryBuilder::insert()
            .table(Self::table_name())
            .columns(vec![
                "ID",
                "TIPO",
                "ALTEZZA",
                "LARGHEZZA",
                "MATERIALE",
                "VETRO",
            ])
            .values(vec![
                item.id.clone().into(),
                item.tipo.clone().into(),
                item.altezza.into(),
                item.larghezza.into(),
                item.materiale.clone().into(),
                item.vetro.clone().into(),
            ]);
        let (query, params) = builder.build()?;

        match conn.execute(
            query.as_str(),
            rusqlite::params_from_iter(convert_param(params)),
        ) {
            Ok(_) => {
                info!("Infisso inserito con successo");
                Ok(item)
            }
            Err(e) => {
                error!("Errore durante l'inserimento {{ infisso }}: {}", e);
                Err(AppError::from(e))
            }
        }
    }
}

impl Update<Infisso> for InfissoDAO {
    fn update<C: DatabaseConnection>(conn: &C, item: Infisso) -> Result<Infisso, AppError> {
        let builder = QueryBuilder::update()
            .table(Self::table_name())
            .set("ALTEZZA", item.altezza)
            .set("LARGHEZZA", item.larghezza)
            .set("MATERIALE", item.materiale.clone())
            .set("VETRO", item.vetro.clone());
        let (query, params) = builder.build()?;

        match conn.execute(
            query.as_str(),
            rusqlite::params_from_iter(convert_param(params)),
        ) {
            Ok(_) => {
                info!("Infisso aggiornato con successo");
                Ok(item)
            }
            Err(e) => {
                error!("Errore durante l'aggiornamento {{ infisso }}: {}", e);
                Err(AppError::from(e))
            }
        }
    }
}
