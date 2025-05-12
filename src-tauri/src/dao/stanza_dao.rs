use crate::dao::crud_operations::{GetAll, Insert, Update};
use crate::dao::entity::Stanza;
use crate::dao::utils::schema_operations::CreateTable;
use crate::dao::utils::DAO;
use crate::database::WhereBuilder;
use crate::database::{convert_param, DatabaseConnection, QueryBuilder, SqlQueryBuilder};
use crate::utils::AppError;
use log::{error, info};

pub struct StanzaDAO;

impl DAO for StanzaDAO {
    fn table_name() -> &'static str {
        "STANZA"
    }
}

impl CreateTable for StanzaDAO {
    fn create_table<C: DatabaseConnection>(conn: &C) -> Result<(), AppError> {
        conn.execute(
            format!("CREATE TABLE IF NOT EXISTS {}
            (
                ID               INTEGER PRIMARY KEY AUTOINCREMENT,
                CHIAVE           TEXT NOT NULL REFERENCES EDIFICIO (CHIAVE),
                PIANO            TEXT NOT NULL,
                ID_SPAZIO        TEXT NOT NULL,
                STANZA           TEXT NOT NULL,
                DESTINAZIONE_USO TEXT NOT NULL,
                ALTEZZA          INTEGER CHECK ( ALTEZZA >= 0 )       DEFAULT 0,
                SPESSORE_MURO    INTEGER CHECK ( SPESSORE_MURO >= 0 ) DEFAULT 0,
                RISCALDAMENTO    TEXT                                 DEFAULT NULL REFERENCES CLIMATIZZAZIONE (CLIMATIZZAZIONE),
                RAFFRESCAMENTO   TEXT                                 DEFAULT NULL REFERENCES CLIMATIZZAZIONE (CLIMATIZZAZIONE),
                ILLUMINAZIONE    TEXT                                 DEFAULT NULL REFERENCES ILLUMINAZIONE (LAMPADINA),
                UNIQUE (CHIAVE, ID_SPAZIO, STANZA, DESTINAZIONE_USO)
            ) STRICT;", Self::table_name()).as_str(),
            ())?;
        info!("Tabella STANZA creata");
        Ok(())
    }
}

impl GetAll<Stanza> for StanzaDAO {
    fn get_all<C: DatabaseConnection>(conn: &C) -> Result<Vec<Stanza>, AppError> {
        let (query, _) = QueryBuilder::select().table(Self::table_name()).build()?;
        let mut stmt = conn.prepare(query.as_str())?;
        let result: Result<Vec<Stanza>, rusqlite::Error> = stmt
            .query_map([], |row| {
                Ok(Stanza {
                    id: row.get::<_, Option<u64>>("ID")?,
                    chiave: row.get::<_, String>("CHIAVE")?,
                    piano: row.get::<_, String>("PIANO")?,
                    id_spazio: row.get::<_, String>("ID_SPAZIO")?,
                    cod_stanza: row.get::<_, String>("STANZA")?,
                    destinazione_uso: row.get::<_, String>("DESTINAZIONE_USO")?,
                    altezza: row.get::<_, Option<u16>>("ALTEZZA")?,
                    spessore_muro: row.get::<_, Option<u8>>("SPESSORE_MURO")?,
                    riscaldamento: row.get::<_, Option<String>>("RISCALDAMENTO")?,
                    raffrescamento: row.get::<_, Option<String>>("RAFFRESCAMENTO")?,
                    illuminazione: row.get::<_, Option<String>>("ILLUMINAZIONE")?,
                })
            })?
            .collect();
        result.map_err(AppError::from)
    }
}

impl Insert<Stanza> for StanzaDAO {
    fn insert<C: DatabaseConnection>(conn: &C, item: Stanza) -> Result<Stanza, AppError> {
        let builder = QueryBuilder::insert()
            .table(Self::table_name())
            .columns(vec![
                "CHIAVE",
                "PIANO",
                "ID_SPAZIO",
                "STANZA",
                "DESTINAZIONE_USO",
                "ALTEZZA",
                "SPESSORE_MURO",
                "RISCALDAMENTO",
                "RAFFRESCAMENTO",
                "ILLUMINAZIONE",
            ])
            .values(vec![
                item.chiave.clone().into(),
                item.piano.clone().into(),
                item.id_spazio.clone().into(),
                item.cod_stanza.clone().into(),
                item.destinazione_uso.clone().into(),
                item.altezza.into(),
                item.spessore_muro.into(),
                item.riscaldamento.clone().into(),
                item.raffrescamento.clone().into(),
                item.illuminazione.clone().into(),
            ]);
        let (query, param) = builder.build()?;

        let mut stmt = conn.prepare(query.as_str())?;
        match stmt.execute(rusqlite::params_from_iter(convert_param(param))) {
            Ok(_) => {
                info!("Stanza inserita con successo");
                Ok(item)
            }
            Err(e) => {
                error!("Errore durante l'inserimento {{ stanza }}: {e}");
                Err(AppError::from(e))
            }
        }
    }
}

impl Update<Stanza> for StanzaDAO {
    fn update<C: DatabaseConnection>(conn: &C, item: Stanza) -> Result<Stanza, AppError> {
        let builder = QueryBuilder::update()
            .table(Self::table_name())
            .set_if("ALTEZZA", item.altezza)
            .set_if("SPESSORE_MURO", item.spessore_muro)
            .set_if("RISCALDAMENTO", item.riscaldamento.clone())
            .set_if("RAFFRESCAMENTO", item.raffrescamento.clone())
            .set_if("ILLUMINAZIONE", item.illuminazione.clone())
            .where_eq("ID", item.id.unwrap());
        let (query, param) = builder.build()?;

        match conn.execute(
            query.as_str(),
            rusqlite::params_from_iter(convert_param(param)),
        ) {
            Ok(_) => {
                info!("Stanza aggiornata con successo");
                Ok(item)
            }
            Err(e) => {
                error!("Errore durante l'aggiornamento {{ stanza }}: {e}");
                Err(AppError::from(e))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rusqlite::Connection;

    fn setup() -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::open_in_memory().unwrap();
        Ok(())
    }

    #[test]
    fn get_infissi_by_id() {}
}
