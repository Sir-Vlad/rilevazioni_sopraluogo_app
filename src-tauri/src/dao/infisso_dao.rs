use crate::dao::entities::entity::Infisso;
use crate::database::{convert_param, QueryBuilder, SqlQueryBuilder};
use crate::dto::InfissoDTO;
use log::{error, info};
use rusqlite::Connection;

pub trait InfissoDAO {
    fn get_all(conn: &Connection) -> Result<Vec<Infisso>, String>;
    fn insert(conn: &Connection, infisso: &InfissoDTO) -> Result<Infisso, String>;
    fn update(conn: &Connection, infisso: &InfissoDTO) -> Result<Infisso, String>;
}

pub struct InfissoDAOImpl;

impl InfissoDAO for InfissoDAOImpl {
    fn get_all(conn: &Connection) -> Result<Vec<Infisso>, String> {
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

    fn insert(conn: &Connection, infisso: &InfissoDTO) -> Result<Infisso, String> {
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
                Ok(Infisso::from(infisso))
            }
            Err(e) => {
                error!("Errore durante l'inserimento {{ infisso }}: {}", e);
                Err(e.to_string())
            }
        }
    }

    #[allow(dead_code, unused_variables)]
    fn update(conn: &Connection, infisso: &InfissoDTO) -> Result<Infisso, String> {
        todo!()
    }
}
