use crate::{
    database::database_manager::ConnectionPool,
    database::Database,
    models::{Edificio, Infisso, Stanza, TipoUtenza},
    schema::{edificio, fotovoltaico, infisso, stanza, stanza_con_infissi, utenze},
};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    Insertable, PgConnection, RunQueryDsl, SqliteConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::{collections::HashMap, error::Error};
use thiserror::Error;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn run_schema_migrations(database: &Database) -> Result<(), MigrationError> {
    let mut conn = database.get_conn()?;
    conn.run_pending_migrations(MIGRATIONS)?;
    log::info!("Migrazioni database eseguite con successo");
    Ok(())
}

#[derive(Error, Debug)]
pub enum MigrationError {
    #[error("Errore di connessione al database: {0}")]
    ConnectionError(#[from] diesel::result::Error),
    #[error("Errore durante l'esecuzione delle migrazioni: {0}")]
    MigrationError(#[from] Box<dyn std::error::Error + Send + Sync>),
}

#[derive(Error, Debug)]
pub enum DataMigrationError {
    #[error("The migration isn't supported yet")]
    UnsupportedMigration,
    #[error("Error diesel: {0}")]
    DieselError(#[from] diesel::result::Error),
    #[error("Error: {0}")]
    GenericError(String),
}

impl From<Box<dyn Error>> for DataMigrationError {
    fn from(value: Box<dyn Error>) -> Self {
        DataMigrationError::GenericError(value.to_string())
    }
}

pub struct DatabaseMigrator<'a> {
    pub from: &'a ConnectionPool,
    pub to: &'a ConnectionPool,
}

impl<'a> DatabaseMigrator<'a> {
    pub fn new(from: &'a ConnectionPool, to: &'a ConnectionPool) -> Self {
        Self { from, to }
    }

    pub fn migrate(&self) -> Result<(), DataMigrationError> {
        match (&self.from, &self.to) {
            (ConnectionPool::Sqlite(from), ConnectionPool::Postgres(to)) => {
                // Copia dati da sqlite a postgres
                let mut conn_sq = from
                    .get()
                    .map_err(|e| DataMigrationError::GenericError(e.to_string()))?;
                let mut conn_pg = to
                    .get()
                    .map_err(|e| DataMigrationError::GenericError(e.to_string()))?;

                Self::migrate_edificio(&mut conn_sq, &mut conn_pg)?;
                log::info!("Migrazione edificio completata");

                Self::migrate_infisso(&mut conn_sq, &mut conn_pg)?;
                log::info!("Migrazione infissi completata");

                let mapping_id = Self::migrate_stanza(&mut conn_sq, &mut conn_pg)?;
                log::info!("Migrazione stanza completata");

                Self::migrate_stanza_con_infissi(&mut conn_sq, &mut conn_pg, mapping_id)?;
                log::info!("Migrazione stanza con infissi completata");

                Self::migrate_fotovoltaico(&mut conn_sq, &mut conn_pg)?;
                log::info!("Migrazione fotovoltaico completata");

                Self::migrate_utenze(&mut conn_sq, &mut conn_pg)?;
                log::info!("Migrazione utenze completata");

                Ok(())
            }
            (ConnectionPool::Postgres(_), ConnectionPool::Sqlite(_)) => {
                Err(DataMigrationError::UnsupportedMigration)
            }
            _ => Err(DataMigrationError::UnsupportedMigration),
        }
    }

    pub fn migrate_edificio(
        from: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
        to: &mut PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Result<(), DataMigrationError> {
        let records: Vec<Edificio> = edificio::table.load::<Edificio>(from)?;
        let records_insert: Vec<EdificioMigration> =
            records.iter().map(EdificioMigration::from).collect();
        if !records.is_empty() {
            diesel::insert_into(edificio::table)
                .values(&records_insert)
                .execute(to)?;
        }
        Ok(())
    }

    pub fn migrate_stanza(
        from: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
        to: &mut PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Result<HashMap<i32, i32>, DataMigrationError> {
        let mut mapping_id: HashMap<i32, i32> = HashMap::new();

        let records: Vec<Stanza> = stanza::table.load::<Stanza>(from)?;
        let records_insert: Vec<StanzaMigration> =
            records.iter().map(StanzaMigration::from).collect();
        if !records.is_empty() {
            let inserted_records: Vec<Stanza> = diesel::insert_into(stanza::table)
                .values(&records_insert)
                .get_results(to)?;

            for (old_stanza, new_stanza) in records.iter().zip(inserted_records.iter()) {
                mapping_id.insert(old_stanza.id, new_stanza.id);
            }
        }

        Ok(mapping_id)
    }

    pub fn migrate_infisso(
        from: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
        to: &mut PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Result<(), DataMigrationError> {
        let records: Vec<Infisso> = infisso::table.load::<Infisso>(from)?;
        let records_insert: Vec<InfissoMigration> =
            records.iter().map(InfissoMigration::from).collect();
        if !records.is_empty() {
            diesel::insert_into(infisso::table)
                .values(&records_insert)
                .execute(to)?;
        }
        Ok(())
    }

    pub fn migrate_stanza_con_infissi(
        from: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
        to: &mut PooledConnection<ConnectionManager<PgConnection>>,
        mapping_id: HashMap<i32, i32>,
    ) -> Result<(), DataMigrationError> {
        let records: Vec<StanzaConInfissiSqlite> =
            diesel::sql_query("SELECT * FROM STANZA_CON_INFISSI").get_results(from)?;

        let records_insert: Vec<StanzaConInfissiMigration<'_>> = records
            .iter()
            .filter_map(|x| {
                if let Some(&stanza_id) = mapping_id.get(&x.stanza_id) {
                    Some(StanzaConInfissiMigration {
                        infisso_id: &x.infisso_id,
                        edificio_id: &x.edificio_id,
                        stanza_id,
                        num_infisso: x.num_infisso,
                    })
                } else {
                    None
                }
            })
            .collect();

        if !records.is_empty() {
            diesel::insert_into(stanza_con_infissi::table)
                .values(&records_insert)
                .execute(to)?;
        }
        Ok(())
    }

    pub fn migrate_fotovoltaico(
        from: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
        to: &mut PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Result<(), DataMigrationError> {
        let records: Vec<FotovoltaicoSqlite> =
            diesel::sql_query("SELECT * FROM FOTOVOLTAICO").get_results(from)?;
        let records_insert: Vec<FotovoltaicoMigration> =
            records.iter().map(FotovoltaicoMigration::from).collect();
        if !records.is_empty() {
            diesel::insert_into(fotovoltaico::table)
                .values(&records_insert)
                .execute(to)?;
        }
        Ok(())
    }

    pub fn migrate_utenze(
        from: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
        to: &mut PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Result<(), DataMigrationError> {
        let records: Vec<UtenzaSqlite> =
            diesel::sql_query("SELECT * FROM UTENZE").get_results(from)?;
        let records_insert: Vec<UtenzaMigration> =
            records.iter().map(UtenzaMigration::from).collect();
        if !records.is_empty() {
            diesel::insert_into(utenze::table)
                .values(&records_insert)
                .execute(to)?;
        }
        Ok(())
    }
}

#[derive(Insertable)]
#[diesel(table_name = edificio)]
struct EdificioMigration<'a> {
    chiave: &'a str,
    fascicolo: i32,
    indirizzo: &'a str,
    anno_costruzione: Option<i32>,
    anno_riqualificazione: Option<i32>,
    note_riqualificazione: Option<&'a str>,
    isolamento_tetto: bool,
    cappotto: bool,
}

impl<'a> From<&'a Edificio> for EdificioMigration<'a> {
    fn from(value: &'a Edificio) -> Self {
        Self {
            chiave: &value.chiave,
            fascicolo: value.fascicolo,
            indirizzo: &value.indirizzo,
            anno_costruzione: value.anno_costruzione,
            anno_riqualificazione: value.anno_riqualificazione,
            note_riqualificazione: value.note_riqualificazione.as_deref(),
            isolamento_tetto: value.isolamento_tetto,
            cappotto: value.cappotto,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = stanza)]
struct StanzaMigration<'a> {
    edificio_id: &'a str,
    piano: &'a str,
    id_spazio: &'a str,
    cod_stanza: &'a str,
    destinazione_uso: &'a str,
    altezza: Option<i16>,
    spessore_muro: Option<i16>,
    riscaldamento: Option<&'a str>,
    raffrescamento: Option<&'a str>,
    illuminazione: Option<&'a str>,
}

impl<'a> From<&'a Stanza> for StanzaMigration<'a> {
    fn from(value: &'a Stanza) -> Self {
        Self {
            edificio_id: &value.edificio_id,
            piano: &value.piano,
            id_spazio: &value.id_spazio,
            cod_stanza: &value.cod_stanza,
            destinazione_uso: &value.destinazione_uso,
            altezza: value.altezza,
            spessore_muro: value.spessore_muro,
            riscaldamento: value.riscaldamento.as_deref(),
            raffrescamento: value.raffrescamento.as_deref(),
            illuminazione: value.illuminazione.as_deref(),
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = infisso)]
struct InfissoMigration<'a> {
    id: &'a str,
    edificio_id: &'a str,
    tipo: &'a str,
    altezza: i16,
    larghezza: i16,
    materiale: &'a str,
    vetro: &'a str,
}

impl<'a> From<&'a Infisso> for InfissoMigration<'a> {
    fn from(value: &'a Infisso) -> Self {
        Self {
            id: &value.id,
            edificio_id: &value.edificio_id,
            tipo: &value.tipo,
            altezza: value.altezza,
            larghezza: value.larghezza,
            materiale: &value.materiale,
            vetro: &value.vetro,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = stanza_con_infissi)]
struct StanzaConInfissiMigration<'a> {
    infisso_id: &'a str,
    edificio_id: &'a str,
    stanza_id: i32, // todo: questo id cambia da sqlite a postegres
    num_infisso: i32,
}

#[derive(Insertable)]
#[diesel(table_name = fotovoltaico)]
struct FotovoltaicoMigration<'a> {
    edificio_id: &'a str,
    potenza: f32,
    proprietario: &'a str,
}

impl<'a> From<&'a FotovoltaicoSqlite> for FotovoltaicoMigration<'a> {
    fn from(value: &'a FotovoltaicoSqlite) -> Self {
        Self {
            edificio_id: &value.edificio_id,
            potenza: value.potenza,
            proprietario: &value.proprietario,
        }
    }
}

#[derive(diesel::QueryableByName, Clone, Debug)]
struct UtenzaSqlite {
    #[diesel(sql_type = diesel::sql_types::Integer, column_name = "ID")]
    pub id: i32,
    #[diesel(sql_type = diesel::sql_types::Text, column_name = "ID_EDIFICIO")]
    pub edificio_id: String,
    #[diesel(sql_type = diesel::sql_types::Text, column_name = "TIPO")]
    pub tipo: String,
    #[diesel(sql_type = diesel::sql_types::Text, column_name = "COD_CONTATORE")]
    pub cod_contatore: String,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>, column_name = "INDIRIZZO_CONTATORE"
    )]
    pub indirizzo_contatore: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = utenze)]
struct UtenzaMigration<'a> {
    edificio_id: &'a str,
    tipo: TipoUtenza,
    cod_contatore: &'a str,
    indirizzo_contatore: Option<&'a str>,
}

impl<'a> From<&'a UtenzaSqlite> for UtenzaMigration<'a> {
    fn from(value: &'a UtenzaSqlite) -> Self {
        Self {
            edificio_id: &value.edificio_id,
            tipo: TipoUtenza::from(value.tipo.as_str()),
            cod_contatore: &value.cod_contatore,
            indirizzo_contatore: value.indirizzo_contatore.as_deref(),
        }
    }
}

#[derive(diesel::QueryableByName, Clone, Debug)]
struct FotovoltaicoSqlite {
    #[diesel(sql_type = diesel::sql_types::Text, column_name = "ID")]
    pub id: String,
    #[diesel(sql_type = diesel::sql_types::Text, column_name = "ID_EDIFICIO")]
    pub edificio_id: String,
    #[diesel(sql_type = diesel::sql_types::Float, column_name = "POTENZA")]
    pub(crate) potenza: f32,
    #[diesel(sql_type = diesel::sql_types::Text, column_name = "PROPRIETARIO")]
    pub(crate) proprietario: String,
}

#[derive(diesel::QueryableByName, Clone, Debug)]
struct StanzaConInfissiSqlite {
    #[diesel(sql_type = diesel::sql_types::Text, column_name = "ID_INFISSO")]
    pub(crate) infisso_id: String,
    #[diesel(sql_type = diesel::sql_types::Text, column_name = "ID_EDIFICIO")]
    pub(crate) edificio_id: String,
    #[diesel(sql_type = diesel::sql_types::Integer, column_name = "ID_STANZA")]
    pub(crate) stanza_id: i32,
    #[diesel(sql_type = diesel::sql_types::Integer, column_name = "NUM_INFISSI")]
    pub(crate) num_infisso: i32,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::models::{Fotovoltaico, NewEdificio, NewInfisso, StanzaConInfissi, Utenza};
    use crate::schema::edificio;
    use diesel::r2d2::Pool;
    use diesel::sql_types::{Float, Integer, Text};
    use diesel::{ExpressionMethods, QueryDsl};
    use tempfile::NamedTempFile;
    use testcontainers::runners::SyncRunner;
    use testcontainers::Container;
    use testcontainers_modules::postgres::Postgres;

    type Result<T = ()> = std::result::Result<T, Box<dyn Error>>;

    struct Setup {
        from: ConnectionPool,
        to: ConnectionPool,
        image: Container<Postgres>,
        temp_file: NamedTempFile,
    }

    fn create_fake_db_sqlite() -> Result<(Pool<ConnectionManager<SqliteConnection>>, NamedTempFile)>
    {
        let tmp_file = NamedTempFile::new()?;
        let path = tmp_file.path().to_str().unwrap();

        let db_sqlite = ConnectionManager::<SqliteConnection>::new(path);
        let pool = Pool::builder().build(db_sqlite)?;
        Ok((pool, tmp_file))
    }

    fn setup() -> Result<Setup> {
        let (pool, tmp_file) = create_fake_db_sqlite()?;

        let image = Postgres::default().start()?;

        let connection_string = &format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            image.get_host_port_ipv4(5432)?
        );
        let db_postgres = ConnectionManager::<PgConnection>::new(connection_string);
        let pool_postgres = Pool::builder().build(db_postgres)?;
        pool_postgres
            .get()?
            .run_pending_migrations(MIGRATIONS)
            .map_err(|e| e.to_string())?;

        Ok(Setup {
            from: ConnectionPool::Sqlite(pool),
            to: ConnectionPool::Postgres(pool_postgres),
            image,
            temp_file: tmp_file,
        })
    }

    #[test]
    fn test_connection_databases() -> Result {
        let Setup { to, from, .. } = setup()?;
        matches!(from, ConnectionPool::Sqlite(_));
        matches!(to, ConnectionPool::Postgres(_));
        Ok(())
    }

    #[test]
    fn test_migrate_edificio() -> Result {
        let Setup {
            to,
            from,
            image: _image,
            temp_file: _tmp_file,
        } = setup()?;
        let mut conn_sq = from.get_sqlite_pool().get()?;
        let mut conn_pg = to.get_postgres_pool().get()?;

        diesel::sql_query(
            "CREATE TABLE IF NOT EXISTS EDIFICIO
                (
                    CHIAVE                TEXT PRIMARY KEY,
                    FASCICOLO             TEXT NOT NULL,
                    INDIRIZZO             TEXT NOT NULL,
                    ANNO_COSTRUZIONE      TEXT    DEFAULT NULL,
                    ANNO_RIQUALIFICAZIONE TEXT    DEFAULT NULL,
                    NOTE_RIQUALIFICAZIONE TEXT    DEFAULT NULL,
                    ISOLAMENTO_TETTO      INTEGER DEFAULT FALSE,
                    CAPPOTTO              INTEGER DEFAULT FALSE
                );",
        )
        .execute(&mut conn_sq)?;

        let edificio = NewEdificio {
            chiave: "785461",
            fascicolo: 78591,
            indirizzo: "Via Roma 123",
        };

        diesel::insert_into(edificio::table)
            .values(&edificio)
            .execute(&mut conn_sq)?;

        diesel::update(edificio::table)
            .set(edificio::anno_riqualificazione.eq::<Option<i32>>(Some(2025)))
            .execute(&mut conn_sq)?;

        DatabaseMigrator::migrate_edificio(&mut conn_sq, &mut conn_pg)?;

        let results: Vec<Edificio> = edificio::table.load(&mut conn_pg)?;
        println!("Dopo: {results:#?}");

        Ok(())
    }

    #[test]
    fn test_migrate_stanza() -> Result {
        todo!()
    }

    #[test]
    fn test_migrate_infisso() -> Result {
        let Setup {
            to,
            from,
            image: _image,
            temp_file: _tmp_file,
        } = setup()?;
        let mut conn_sq = from.get_sqlite_pool().get()?;
        let mut conn_pg = to.get_postgres_pool().get()?;

        diesel::sql_query("PRAGMA foreign_keys = off").execute(&mut conn_sq)?;
        //diesel::sql_query("SET session_replication_role = replica").execute(&mut conn_pg)?;

        diesel::sql_query(
            "CREATE TABLE IF NOT EXISTS INFISSO
            (
                ID        TEXT,
                EDIFICIO_ID  TEXT    NOT NULL REFERENCES EDIFICIO (CHIAVE),
                TIPO      TEXT    NOT NULL REFERENCES TIPO_INFISSO (NOME),
                ALTEZZA   INTEGER NOT NULL CHECK ( ALTEZZA >= 0 ),
                LARGHEZZA INTEGER NOT NULL CHECK ( LARGHEZZA >= 0 ),
                MATERIALE TEXT    NOT NULL REFERENCES MATERIALE_INFISSO (MATERIALE),
                VETRO     TEXT    NOT NULL REFERENCES VETRO_INFISSO (VETRO),
                MQ        REAL GENERATED ALWAYS AS ((ALTEZZA * LARGHEZZA) / 10000.0) VIRTUAL,
                PRIMARY KEY (ID, EDIFICIO_ID),
                UNIQUE (ID, EDIFICIO_ID, TIPO, ALTEZZA, LARGHEZZA, MATERIALE, VETRO)
            );",
        )
        .execute(&mut conn_sq)?;

        let infisso = NewInfisso {
            id: "A",
            edificio_id: "785461",
            tipo: "Porta",
            altezza: 120,
            larghezza: 150,
            materiale: "Legno",
            vetro: "Singolo",
        };

        diesel::insert_into(infisso::table)
            .values(&infisso)
            .execute(&mut conn_sq)?;

        diesel::insert_into(edificio::table)
            .values(&NewEdificio {
                chiave: infisso.edificio_id,
                fascicolo: 78591,
                indirizzo: "Via Roma 123",
            })
            .execute(&mut conn_pg)?;

        DatabaseMigrator::migrate_infisso(&mut conn_sq, &mut conn_pg)?;

        let results: Vec<Infisso> = infisso::table.load(&mut conn_pg)?;
        println!("Dopo: {results:#?}");

        Ok(())
    }

    #[test]
    fn test_migrate_stanze_con_infissi() -> Result {
        let Setup {
            to,
            from,
            image: _image,
            temp_file: _tmp_file1,
        } = setup()?;
        let mut conn_sq1 = from.get_sqlite_pool().get()?;
        let mut conn_pg = to.get_postgres_pool().get()?;

        let (pool, _tmp_file2) = create_fake_db_sqlite()?;
        let mut conn_sq2 = pool.get()?;

        // disable foreign key
        diesel::sql_query("PRAGMA foreign_keys = off").execute(&mut conn_sq1)?;
        diesel::sql_query("PRAGMA foreign_keys = off").execute(&mut conn_sq2)?;
        diesel::sql_query("SET session_replication_role = replica").execute(&mut conn_pg)?;

        // create table into database sqlite
        let query_create_table = "CREATE TABLE IF NOT EXISTS STANZA_CON_INFISSI
                (
                    ID_STANZA      INTEGER NOT NULL REFERENCES STANZA (ID),
                    ID_INFISSO     TEXT    NOT NULL,
                    ID_EDIFICIO    TEXT    NOT NULL,
                    NUM_INFISSI    INTEGER NOT NULL DEFAULT 1 CHECK ( NUM_INFISSI > 0 ),
                    PRIMARY KEY (ID_INFISSO, ID_STANZA, ID_EDIFICIO),
                    FOREIGN KEY (ID_INFISSO, ID_EDIFICIO) REFERENCES INFISSO (ID, EDIFICIO)
                )";
        diesel::sql_query(query_create_table).execute(&mut conn_sq1)?;
        diesel::sql_query(query_create_table).execute(&mut conn_sq2)?;

        let value_insert = [
            StanzaConInfissi {
                infisso_id: "A".to_string(),
                edificio_id: "7878788".to_string(),
                stanza_id: 7,
                num_infisso: 25,
            },
            StanzaConInfissi {
                infisso_id: "B".to_string(),
                edificio_id: "7878788".to_string(),
                stanza_id: 8,
                num_infisso: 12,
            },
        ];

        let insert_value = |conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
                            value: &StanzaConInfissi|
         -> Result {
            diesel::sql_query(
            "INSERT INTO STANZA_CON_INFISSI (ID_STANZA, ID_INFISSO, ID_EDIFICIO, NUM_INFISSI) VALUES ($1, $2, $3, $4)",
        )
            .bind::<Integer, _>(value.stanza_id)
            .bind::<Text, _>(&value.infisso_id)
            .bind::<Text, _>(&value.edificio_id)
            .bind::<Integer, _>(value.num_infisso)
            .execute(conn)?;
            Ok(())
        };

        for value in value_insert.iter() {
            insert_value(&mut conn_sq1, value)?;
            insert_value(&mut conn_sq2, value)?;
        }

        let mapping_id = HashMap::from([(7, 1), (8, 2)]);
        DatabaseMigrator::migrate_stanza_con_infissi(&mut conn_sq1, &mut conn_pg, mapping_id)?;

        let mapping_id = HashMap::from([(7, 3), (8, 4)]);
        DatabaseMigrator::migrate_stanza_con_infissi(&mut conn_sq2, &mut conn_pg, mapping_id)?;

        let results: Vec<StanzaConInfissi> = stanza_con_infissi::table.load(&mut conn_pg)?;
        println!("Dopo: {results:#?}");

        Ok(())
    }

    #[test]
    fn test_migrate_utenze() -> Result {
        let Setup {
            to,
            from,
            image: _image,
            temp_file: _tmp_file,
        } = setup()?;
        let mut conn_sq = from.get_sqlite_pool().get()?;
        let mut conn_pg = to.get_postgres_pool().get()?;

        diesel::sql_query("PRAGMA foreign_keys = off").execute(&mut conn_sq)?;
        diesel::sql_query("SET session_replication_role = replica").execute(&mut conn_pg)?;

        diesel::sql_query("CREATE TABLE IF NOT EXISTS UTENZE
            (
                ID                  INTEGER PRIMARY KEY AUTOINCREMENT,
                ID_EDIFICIO         TEXT NOT NULL REFERENCES EDIFICIO (CHIAVE),
                TIPO                TEXT NOT NULL CHECK ( TIPO IN ('idrica', 'termica', 'elettrica') ),
                COD_CONTATORE       TEXT NOT NULL,
                INDIRIZZO_CONTATORE TEXT
            );").execute(&mut conn_sq)?;

        diesel::sql_query("INSERT INTO UTENZE (ID_EDIFICIO, TIPO, COD_CONTATORE, INDIRIZZO_CONTATORE) VALUES ($1, $2, $3, $4)")
            .bind::<Text, _>("454545")
            .bind::<Text, _>("idrica")
            .bind::<Text, _>("785461")
            .bind::<Text, _>("Via Roma 123")
            .execute(&mut conn_sq)?;

        DatabaseMigrator::migrate_utenze(&mut conn_sq, &mut conn_pg)?;

        let results: Vec<Utenza> = utenze::table.load(&mut conn_pg)?;
        println!("Dopo: {results:#?}");

        Ok(())
    }

    #[test]
    fn test_migrate_fotovoltaico() -> Result {
        let Setup {
            to,
            from,
            image: _image,
            temp_file: _tmp_file,
        } = setup()?;
        let mut conn_sq = from.get_sqlite_pool().get()?;
        let mut conn_pg = to.get_postgres_pool().get()?;

        diesel::sql_query("PRAGMA foreign_keys = off").execute(&mut conn_sq)?;
        diesel::sql_query("SET session_replication_role = replica").execute(&mut conn_pg)?;

        diesel::sql_query(
            "CREATE TABLE IF NOT EXISTS FOTOVOLTAICO
                (
                    ID           INTEGER PRIMARY KEY AUTOINCREMENT,
                    ID_EDIFICIO  TEXT REFERENCES EDIFICIO (CHIAVE),
                    POTENZA      REAL NOT NULL CHECK ( POTENZA >= 0 ),
                    PROPRIETARIO TEXT NOT NULL
                );",
        )
        .execute(&mut conn_sq)?;

        diesel::sql_query(
            "INSERT INTO FOTOVOLTAICO (ID_EDIFICIO, POTENZA, PROPRIETARIO) VALUES ($1, $2, $3)",
        )
        .bind::<Text, _>("454545")
        .bind::<Float, _>(30.9)
        .bind::<Text, _>("Ugo Ugolini")
        .execute(&mut conn_sq)?;

        DatabaseMigrator::migrate_fotovoltaico(&mut conn_sq, &mut conn_pg)?;

        let results: Vec<Fotovoltaico> = fotovoltaico::table.load(&mut conn_pg)?;
        println!("Dopo: {results:#?}");

        Ok(())
    }

    #[test]
    fn test_full_migration_with_real_data() -> Result {
        let Setup {
            to,
            from,
            image: _image,
            temp_file: _tmp_file,
        } = setup()?;

        let mut conn_sq = from.get_sqlite_pool().get()?;
        let mut conn_pg = to.get_postgres_pool().get()?;

        // Disabilita foreign keys per i test
        diesel::sql_query("PRAGMA foreign_keys = off").execute(&mut conn_sq)?;
        //diesel::sql_query("SET session_replication_role = replica").execute(&mut conn_pg)?;

        // Crea e popola database SQLite con dati realistici
        setup_realistic_sqlite_data(&mut conn_sq)?;

        // Esegui la migrazione completa
        let migration = DatabaseMigrator::new(&from, &to);
        migration.migrate()?;

        // Verifica che i dati siano stati migrati correttamente
        verify_migration_results(&mut conn_pg)?;

        Ok(())
    }

    fn setup_realistic_sqlite_data(
        conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    ) -> Result {
        // Crea le tabelle SQLite
        create_sqlite_tables(conn)?;

        // Inserisci dati realistici
        insert_realistic_data(conn)?;

        Ok(())
    }

    fn create_sqlite_tables(
        conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    ) -> Result {
        // Edificio
        diesel::sql_query(
            "CREATE TABLE IF NOT EXISTS EDIFICIO (
                CHIAVE                TEXT PRIMARY KEY,
                FASCICOLO             TEXT NOT NULL,
                INDIRIZZO             TEXT NOT NULL,
                ANNO_COSTRUZIONE      TEXT    DEFAULT NULL,
                ANNO_RIQUALIFICAZIONE TEXT    DEFAULT NULL,
                NOTE_RIQUALIFICAZIONE TEXT    DEFAULT NULL,
                ISOLAMENTO_TETTO      INTEGER DEFAULT FALSE,
                CAPPOTTO              INTEGER DEFAULT FALSE
            );",
        )
        .execute(conn)?;

        // Stanza
        diesel::sql_query(
            "CREATE TABLE IF NOT EXISTS STANZA (
                ID                INTEGER PRIMARY KEY AUTOINCREMENT,
                EDIFICIO_ID       TEXT NOT NULL REFERENCES EDIFICIO (CHIAVE),
                PIANO             TEXT NOT NULL,
                ID_SPAZIO         TEXT NOT NULL,
                COD_STANZA        TEXT NOT NULL,
                DESTINAZIONE_USO  TEXT NOT NULL,
                ALTEZZA           INTEGER,
                SPESSORE_MURO     INTEGER,
                RISCALDAMENTO     TEXT,
                RAFFRESCAMENTO    TEXT,
                ILLUMINAZIONE     TEXT
            );",
        )
        .execute(conn)?;

        // Infisso
        diesel::sql_query(
            "CREATE TABLE IF NOT EXISTS INFISSO (
                ID        TEXT,
                EDIFICIO_ID  TEXT    NOT NULL REFERENCES EDIFICIO (CHIAVE),
                TIPO      TEXT    NOT NULL,
                ALTEZZA   INTEGER NOT NULL CHECK ( ALTEZZA >= 0 ),
                LARGHEZZA INTEGER NOT NULL CHECK ( LARGHEZZA >= 0 ),
                MATERIALE TEXT    NOT NULL,
                VETRO     TEXT    NOT NULL,
                MQ        REAL GENERATED ALWAYS AS ((ALTEZZA * LARGHEZZA) / 10000.0) VIRTUAL,
                PRIMARY KEY (ID, EDIFICIO_ID)
            );",
        )
        .execute(conn)?;

        // Stanza con infissi
        diesel::sql_query(
            "CREATE TABLE IF NOT EXISTS STANZA_CON_INFISSI (
                ID_STANZA      INTEGER NOT NULL REFERENCES STANZA (ID),
                ID_INFISSO     TEXT    NOT NULL,
                ID_EDIFICIO    TEXT    NOT NULL,
                NUM_INFISSI    INTEGER NOT NULL DEFAULT 1 CHECK ( NUM_INFISSI > 0 ),
                PRIMARY KEY (ID_INFISSO, ID_STANZA, ID_EDIFICIO),
                FOREIGN KEY (ID_INFISSO, ID_EDIFICIO) REFERENCES INFISSO (ID, EDIFICIO)
            );",
        )
        .execute(conn)?;

        // Utenze
        diesel::sql_query(
            "CREATE TABLE IF NOT EXISTS UTENZE (
                ID                  INTEGER PRIMARY KEY AUTOINCREMENT,
                ID_EDIFICIO         TEXT NOT NULL REFERENCES EDIFICIO (CHIAVE),
                TIPO                TEXT NOT NULL CHECK ( TIPO IN ('idrica', 'termica', 'elettrica') ),
                COD_CONTATORE       TEXT NOT NULL,
                INDIRIZZO_CONTATORE TEXT
            );"
        ).execute(conn)?;

        // Fotovoltaico
        diesel::sql_query(
            "CREATE TABLE IF NOT EXISTS FOTOVOLTAICO (
                ID           INTEGER PRIMARY KEY AUTOINCREMENT,
                ID_EDIFICIO  TEXT REFERENCES EDIFICIO (CHIAVE),
                POTENZA      REAL NOT NULL CHECK ( POTENZA >= 0 ),
                PROPRIETARIO TEXT NOT NULL
            );",
        )
        .execute(conn)?;

        Ok(())
    }

    fn insert_realistic_data(
        conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    ) -> Result {
        // Inserisci edifici
        diesel::sql_query(
            "INSERT INTO EDIFICIO (CHIAVE, FASCICOLO, INDIRIZZO, ANNO_COSTRUZIONE, ISOLAMENTO_TETTO, CAPPOTTO) VALUES
            ('EDI001', '12345', 'Via Roma 123', '1985', 1, 0),
            ('EDI002', '12346', 'Via Milano 456', '1990', 0, 1),
            ('EDI003', '12347', 'Via Napoli 789', '2000', 1, 1);"
        ).execute(conn)?;

        // Inserisci stanze
        diesel::sql_query(
            "INSERT INTO STANZA (EDIFICIO_ID, PIANO, ID_SPAZIO, COD_STANZA, DESTINAZIONE_USO, ALTEZZA, SPESSORE_MURO) VALUES
            ('EDI001', 'T', 'SP001', 'S001', 'Ufficio', 280, 20),
            ('EDI001', 'T', 'SP002', 'S002', 'Bagno', 260, 15),
            ('EDI001', '1', 'SP003', 'S003', 'Sala riunioni', 300, 25),
            ('EDI002', 'T', 'SP001', 'S001', 'Reception', 320, 20),
            ('EDI002', '1', 'SP002', 'S002', 'Ufficio', 280, 18);"
        ).execute(conn)?;

        // Inserisci infissi
        diesel::sql_query(
            "INSERT INTO INFISSO (ID, EDIFICIO_ID, TIPO, ALTEZZA, LARGHEZZA, MATERIALE, VETRO) VALUES
            ('A', 'EDI001', 'Porta', 2100, 800, 'Legno', 'Singolo'),
            ('B', 'EDI001', 'Finestra', 1200, 1000, 'PVC', 'Doppio'),
            ('C', 'EDI001', 'Finestra', 1100, 800, 'Alluminio', 'Triplo'),
            ('A', 'EDI002', 'Porta', 2000, 900, 'Legno', 'Singolo'),
            ('B', 'EDI002', 'Finestra', 1300, 1200, 'PVC', 'Doppio');"
        ).execute(conn)?;

        // Inserisci stanza con infissi
        diesel::sql_query(
            "INSERT INTO STANZA_CON_INFISSI (ID_STANZA, ID_INFISSO, ID_EDIFICIO, NUM_INFISSI) VALUES
            (1, 'A', 'EDI001', 1),
            (1, 'B', 'EDI001', 2),
            (2, 'C', 'EDI001', 1),
            (3, 'B', 'EDI001', 3),
            (4, 'A', 'EDI002', 1),
            (5, 'B', 'EDI002', 2);"
        ).execute(conn)?;

        // Inserisci utenze
        diesel::sql_query(
            "INSERT INTO UTENZE (ID_EDIFICIO, TIPO, COD_CONTATORE, INDIRIZZO_CONTATORE) VALUES
            ('EDI001', 'elettrica', 'ELE001234', 'Via Roma 123'),
            ('EDI001', 'idrica', 'IDR001234', 'Via Roma 123'),
            ('EDI001', 'termica', 'TER001234', 'Via Roma 123'),
            ('EDI002', 'elettrica', 'ELE005678', 'Via Milano 456'),
            ('EDI002', 'idrica', 'IDR005678', 'Via Milano 456');",
        )
        .execute(conn)?;

        // Inserisci fotovoltaico
        diesel::sql_query(
            "INSERT INTO FOTOVOLTAICO (ID_EDIFICIO, POTENZA, PROPRIETARIO) VALUES
            ('EDI001', 15.5, 'Mario Rossi'),
            ('EDI002', 22.3, 'Lucia Bianchi'),
            ('EDI003', 8.7, 'Giuseppe Verdi');",
        )
        .execute(conn)?;

        Ok(())
    }

    fn verify_migration_results(
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Result {
        // Verifica edifici
        let edifici: Vec<Edificio> = edificio::table.load(conn)?;
        assert_eq!(edifici.len(), 3);
        println!("Edifici migrati: {}", edifici.len());

        // Verifica stanze
        let stanze: Vec<Stanza> = stanza::table.load(conn)?;
        assert_eq!(stanze.len(), 5);
        println!("Stanze migrate: {}", stanze.len());

        // Verifica infissi
        let infissi: Vec<Infisso> = infisso::table.load(conn)?;
        assert_eq!(infissi.len(), 5);
        println!("Infissi migrati: {}", infissi.len());

        // Verifica stanza con infissi
        let stanza_infissi: Vec<StanzaConInfissi> = stanza_con_infissi::table.load(conn)?;
        assert_eq!(stanza_infissi.len(), 6);
        println!("Stanza con infissi migrati: {}", stanza_infissi.len());

        // Verifica utenze
        let utenze: Vec<Utenza> = utenze::table.load(conn)?;
        assert_eq!(utenze.len(), 5);
        println!("Utenze migrate: {}", utenze.len());

        // Verifica fotovoltaico
        let fotovoltaico: Vec<Fotovoltaico> = fotovoltaico::table.load(conn)?;
        assert_eq!(fotovoltaico.len(), 3);
        println!("Fotovoltaico migrato: {}", fotovoltaico.len());

        // Verifica che i mapping degli ID siano corretti
        let stanza_prima = stanze
            .iter()
            .find(|s| s.cod_stanza == "S001" && s.edificio_id.trim() == "EDI001")
            .unwrap();
        let infissi_stanza = stanza_con_infissi::table
            .filter(stanza_con_infissi::stanza_id.eq(stanza_prima.id))
            .load::<StanzaConInfissi>(conn)?;
        assert!(!infissi_stanza.is_empty());
        println!("Mapping ID verificato correttamente");

        Ok(())
    }
}
