use crate::dao::crud_operations::{GetAll, Insert};
use crate::dao::entity::Illuminazione;
use crate::dao::utils::schema_operations::CreateTable;
use crate::dao::utils::DAO;
use crate::database::{convert_param, DatabaseConnection, QueryBuilder, SqlQueryBuilder};
use crate::utils::AppError;
use log::info;

pub struct IlluminazioneDAO;

impl DAO for IlluminazioneDAO {
    fn table_name() -> &'static str {
        "ILLUMINAZIONE"
    }
}

impl CreateTable for IlluminazioneDAO {
    fn create_table<C: DatabaseConnection>(conn: &C) -> Result<(), AppError> {
        conn.execute(
            format!(
                "CREATE TABLE IF NOT EXISTS {}
                    (
                        ID                    INTEGER PRIMARY KEY AUTOINCREMENT,
                        LAMPADINA             TEXT    NOT NULL UNIQUE COLLATE NOCASE,
                        EFFICIENZA_ENERGETICA INTEGER NOT NULL
                    ) STRICT;",
                Self::table_name()
            )
            .as_str(),
            (),
        )?;
        info!("Tabella ILLUMINAZIONE creata");
        Ok(())
    }
}

impl GetAll<Illuminazione> for IlluminazioneDAO {
    fn get_all<C: DatabaseConnection>(conn: &C) -> Result<Vec<Illuminazione>, AppError> {
        let (query, _) = QueryBuilder::select().table(Self::table_name()).build()?;

        let mut stmt = conn.prepare(query.as_str())?;

        let result: Result<Vec<Illuminazione>, rusqlite::Error> = stmt
            .query_map([], |row| {
                Ok(Illuminazione {
                    _id: Some(row.get::<_, u64>("ID")?),
                    lampadina: row.get::<_, String>("LAMPADINA")?,
                    efficienza_energetica: row.get::<_, u8>("EFFICIENZA_ENERGETICA")?,
                })
            })
            .expect("Errore nella lettura dei dati di tipo materiale")
            .collect();
        result.map_err(|e| AppError::from(e))
    }
}

impl Insert<Illuminazione> for IlluminazioneDAO {
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
