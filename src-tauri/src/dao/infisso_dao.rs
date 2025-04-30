use crate::dao::crud_operations::{GetAll, Insert, Update};
use crate::dao::entity::Infisso;
use crate::dao::utils::schema_operations::CreateTable;
use crate::database::{convert_param, DatabaseConnection, QueryBuilder, SqlQueryBuilder};
use log::{error, info};

pub struct InfissoDAO;

impl CreateTable for InfissoDAO {
    fn create_table<C: DatabaseConnection>(conn: &C) -> Result<(), String> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS INFISSO
            (
                ID        TEXT PRIMARY KEY,
                TIPO      TEXT    NOT NULL CHECK ( TIPO IN ('PORTA', 'FINESTRA') ) DEFAULT 'FINESTRA',
                ALTEZZA   INTEGER NOT NULL CHECK ( ALTEZZA >= 0 ),
                LARGHEZZA INTEGER NOT NULL CHECK ( LARGHEZZA >= 0 ),
                MATERIALE TEXT    NOT NULL REFERENCES MATERIALE_INFISSO (MATERIALE),
                VETRO     TEXT    NOT NULL REFERENCES VETRO_INFISSO (VETRO),
                MQ        REAL GENERATED ALWAYS AS ((ALTEZZA * LARGHEZZA) / 10000.0) VIRTUAL,
                UNIQUE (TIPO, ALTEZZA, LARGHEZZA, MATERIALE, VETRO)
            ) STRICT;"
            ,()).map_err(|e| e.to_string())?;
        info!("Tabella INFISSO creata");
        Ok(())
    }
}

impl GetAll<Infisso> for InfissoDAO {
    fn get_all<C: DatabaseConnection>(conn: &C) -> Result<Vec<Infisso>, String> {
        let (query, _) = QueryBuilder::select()
            .table("INFISSO")
            .build()
            .map_err(|e| e.to_string())?;

        let mut stmt = conn
            .prepare(query.as_str())
            .map_err(|e| e.to_string())
            .ok()
            .unwrap();
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
            })
            .expect("Errore nella lettura dei dati dal database")
            .collect();

        match infissi {
            Ok(infissi) => Ok(infissi),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl Insert<Infisso> for InfissoDAO {
    fn insert<C: DatabaseConnection>(conn: &C, infisso: Infisso) -> Result<Infisso, String> {
        let builder = QueryBuilder::insert()
            .table("INFISSO")
            .columns(vec![
                "ID",
                "TIPO",
                "ALTEZZA",
                "LARGHEZZA",
                "MATERIALE",
                "VETRO",
            ])
            .values(vec![
                infisso.id.clone().into(),
                infisso.tipo.clone().into(),
                infisso.altezza.into(),
                infisso.larghezza.into(),
                infisso.materiale.clone().into(),
                infisso.vetro.clone().into(),
            ]);
        let (query, params) = builder.build().map_err(|e| e.to_string())?;

        match conn
            .execute(
                query.as_str(),
                rusqlite::params_from_iter(convert_param(params)),
            )
            .map_err(|e| e.to_string())
        {
            Ok(_) => {
                info!("Infisso inserito con successo");
                Ok(infisso)
            }
            Err(e) => {
                error!("Errore durante l'inserimento {{ infisso }}: {}", e);
                Err(e.to_string())
            }
        }
    }
}

impl Update<Infisso> for InfissoDAO {
    fn update<C: DatabaseConnection>(conn: &C, infisso: Infisso) -> Result<Infisso, String> {
        let builder = QueryBuilder::update()
            .table("INFISSO")
            .set("ALTEZZA", infisso.altezza)
            .set("LARGHEZZA", infisso.larghezza)
            .set("MATERIALE", infisso.materiale.clone())
            .set("VETRO", infisso.vetro.clone());
        let (query, params) = builder.build().map_err(|e| e.to_string())?;

        match conn.execute(
            query.as_str(),
            rusqlite::params_from_iter(convert_param(params)),
        ) {
            Ok(_) => {
                info!("Infisso aggiornato con successo");
                Ok(infisso)
            }
            Err(e) => {
                error!("Errore durante l'aggiornamento {{ infisso }}: {}", e);
                Err(e.to_string())
            }
        }
    }
}
