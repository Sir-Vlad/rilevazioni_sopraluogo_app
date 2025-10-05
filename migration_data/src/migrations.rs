use std::collections::HashMap;

use app_models::{
    models::{Edificio, Fotovoltaico, Infisso, Stanza, StanzaConInfissi, TipoUtenza},
    schema::{edificio, fotovoltaico, infisso, stanza, stanza_con_infissi, utenze},
};
use diesel::{
    Insertable, PgConnection, RunQueryDsl, SqliteConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};

use crate::errors::{DataMigrationError, MigrationError};

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;
pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

#[derive(Debug)]
pub enum ConnectionPool {
    Sqlite(SqlitePool),
    Postgres(PostgresPool),
}

impl ConnectionPool {
    pub fn get_sqlite_pool(&self) -> &SqlitePool {
        match self {
            ConnectionPool::Sqlite(pool) => pool,
            _ => panic!("Attempted to get SQLite pool from Postgres pool"),
        }
    }

    pub fn get_postgres_pool(&self) -> &PostgresPool {
        match self {
            ConnectionPool::Postgres(pool) => pool,
            _ => panic!("Attempted to get Postgres pool from Sqlite pool"),
        }
    }
}

pub struct DatabaseMigrator<'a> {
    pub from: &'a ConnectionPool,
    pub to: &'a ConnectionPool,
}

impl<'a> DatabaseMigrator<'a> {
    pub fn new(from: &'a ConnectionPool, to: &'a ConnectionPool) -> Self { Self { from, to } }

    /// Migrates data between two database connections defined in the
    /// `ConnectionPool`.
    ///
    /// # Description
    /// This function handles the migration of data from one database type to
    /// another. Currently, only migrations from SQLite
    /// (`ConnectionPool::Sqlite`) to PostgreSQL (`ConnectionPool::Postgres`)
    /// are supported. The migration process is divided into multiple steps,
    /// each responsible for migrating a specific part of the database
    /// schema, such as buildings (`edificio`), windows (`infissi`),
    /// rooms (`stanza`), and other entities.
    ///
    /// The steps involved in the migration are:
    /// 1. `migrate_edificio`: Migrates building (`edificio`) data.
    /// 2. `migrate_infisso`: Migrates window (`infisso`) data.
    /// 3. `migrate_stanza`: Migrates room (`stanza`) data and returns a mapping
    ///    of IDs for further steps.
    /// 4. `migrate_stanza_con_infissi`: Migrates rooms associated with windows,
    ///    using the mapping obtained in step 3.
    /// 5. `migrate_fotovoltaico`: Migrates photovoltaic (`fotovoltaico`) data.
    /// 6. `migrate_utenze`: Migrates utility (`utenze`) data.
    ///
    /// Messages are logged at each step to indicate the progress of the
    /// migration process.
    ///
    /// # Errors
    /// Returns a `Result` where:
    /// - `Ok(())` indicates a successful migration.
    /// - `Err(MigrationError)` is returned in the following scenarios:
    ///   - If the source and destination connection types do not match the
    ///     supported migration (e.g., PostgreSQL to SQLite).
    ///   - If there is an error when acquiring connections from the database
    ///     pools.
    ///   - If there is a failure in one of the specific migration steps.
    ///
    /// # Notes
    /// - This function assumes that both the source (`from`) and destination
    ///   (`to`) connection pools are valid and operational.
    /// - Attempting to migrate in unsupported directions (e.g., PostgreSQL to
    ///   SQLite) will result in a `DataMigrationError::UnsupportedMigration`
    ///   error being returned.
    pub fn migrate(&self) -> Result<(), MigrationError> {
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
                tracing::info!("Migrazione edificio completata");

                Self::migrate_infisso(&mut conn_sq, &mut conn_pg)?;
                tracing::info!("Migrazione infissi completata");

                let mapping_id = Self::migrate_stanza(&mut conn_sq, &mut conn_pg)?;
                tracing::info!("Migrazione stanza completata");

                Self::migrate_stanza_con_infissi(&mut conn_sq, &mut conn_pg, mapping_id)?;
                tracing::info!("Migrazione stanza con infissi completata");

                Self::migrate_fotovoltaico(&mut conn_sq, &mut conn_pg)?;
                tracing::info!("Migrazione fotovoltaico completata");

                Self::migrate_utenze(&mut conn_sq, &mut conn_pg)?;
                tracing::info!("Migrazione utenze completata");

                Ok(())
            }
            (ConnectionPool::Postgres(_), ConnectionPool::Sqlite(_)) => Err(MigrationError::from(
                DataMigrationError::UnsupportedMigration,
            )),
            _ => Err(MigrationError::from(
                DataMigrationError::UnsupportedMigration,
            )),
        }
    }

    fn migrate_edificio(
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

    fn migrate_stanza(
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

    fn migrate_infisso(
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

    fn migrate_stanza_con_infissi(
        from: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
        to: &mut PooledConnection<ConnectionManager<PgConnection>>,
        mapping_id: HashMap<i32, i32>,
    ) -> Result<(), DataMigrationError> {
        let records: Vec<StanzaConInfissi> = stanza_con_infissi::table.load(from)?;
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

    fn migrate_fotovoltaico(
        from: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
        to: &mut PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Result<(), DataMigrationError> {
        let records: Vec<Fotovoltaico> = fotovoltaico::table.load(from)?;
        let records_insert: Vec<FotovoltaicoMigration> =
            records.iter().map(FotovoltaicoMigration::from).collect();
        if !records.is_empty() {
            diesel::insert_into(fotovoltaico::table)
                .values(&records_insert)
                .execute(to)?;
        }
        Ok(())
    }

    fn migrate_utenze(
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
    riscaldamento: Option<String>,
    raffrescamento: Option<String>,
    illuminazione: Option<String>,
}

fn check_climatizzazione(value: &Option<String>) -> Option<String> {
    if value.is_none() {
        return None;
    }
    let value = capitalize(value.clone().unwrap());

    match value.as_str() {
        "No climatizzata" => Some(value),
        "Radiatori" => Some(value),
        "Ventilconvettori" => Some(value),
        "Split" => Some(value),
        "A pavimento" => Some(value),
        "Pannelli radianti" => Some(value),
        "Bocchette ad aria" => Some(value),
        _ => {
            panic!("Climatizzazione non valida: {value}")
        }
    }
}

fn check_illuminazione(value: &Option<String>) -> Option<String> {
    if value.is_none() {
        return None;
    }
    let value = capitalize(value.clone().unwrap());

    match value.as_str() {
        "No illuminata" => Some(value),
        "Alogene" => Some(value),
        "Neon" => Some(value),
        "Led" => Some(value),
        "Fluorescenza" => Some(value),
        _ => {
            panic!("Illuminazione non valida: {value}")
        }
    }
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
            riscaldamento: check_climatizzazione(&value.riscaldamento),
            raffrescamento: check_climatizzazione(&value.raffrescamento),
            illuminazione: check_illuminazione(&value.illuminazione),
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = infisso)]
struct InfissoMigration<'a> {
    id: &'a str,
    edificio_id: &'a str,
    tipo: String,
    altezza: i16,
    larghezza: i16,
    materiale: String,
    vetro: String,
}

fn check_vetro_infisso(value: &str) -> String {
    let value = capitalize(value.to_string());
    match value.as_str() {
        "Singolo" => value,
        "Doppio" => value,
        "Camera" => value,
        "Triplo" => value,
        "Plexiglas" => value,
        _ => {
            panic!("Vetro non valido: {value}")
        }
    }
}

fn check_materiale_infisso(value: &str) -> String {
    let value = capitalize(value.to_string());
    match value.as_str() {
        "Legno" => value,
        "Ferro" => value,
        "Alluminio" => value,
        "Pvc" => "PVC".to_string(),
        _ => {
            panic!("Vetro non valido: {value}")
        }
    }
}

impl<'a> From<&'a Infisso> for InfissoMigration<'a> {
    fn from(value: &'a Infisso) -> Self {
        Self {
            id: &value.id,
            edificio_id: &value.edificio_id,
            tipo: capitalize(value.tipo.clone()),
            altezza: value.altezza,
            larghezza: value.larghezza,
            materiale: check_materiale_infisso(&value.materiale),
            vetro: check_vetro_infisso(&value.vetro),
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = stanza_con_infissi)]
struct StanzaConInfissiMigration<'a> {
    infisso_id: &'a str,
    edificio_id: &'a str,
    stanza_id: i32,
    num_infisso: i32,
}

#[derive(Insertable)]
#[diesel(table_name = fotovoltaico)]
struct FotovoltaicoMigration<'a> {
    edificio_id: &'a str,
    potenza: f32,
    proprietario: &'a str,
}

impl<'a> From<&'a Fotovoltaico> for FotovoltaicoMigration<'a> {
    fn from(value: &'a Fotovoltaico) -> Self {
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
    #[diesel(sql_type = diesel::sql_types::Text, column_name = "EDIFICIO_ID")]
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

#[inline]
fn capitalize(s: String) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            first.to_uppercase().collect::<String>() + &chars.collect::<String>().to_lowercase()
        }
    }
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use app_models::{
        MIGRATIONS_POSTGRES,
        models::{NewEdificio, NewFotovoltaico, NewInfisso, NewStanza, Utenza},
    };
    use diesel::{ExpressionMethods, sql_types::Text};
    use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
    use tempfile::NamedTempFile;
    use testcontainers::{Container, runners::SyncRunner};
    use testcontainers_modules::postgres::Postgres;

    use super::*;

    const MIGRATIONS_SQLITE: EmbeddedMigrations = embed_migrations!("./migrations/sqlite");

    type Result<T = ()> = std::result::Result<T, Box<dyn Error>>;
    type PostgresPool = Pool<ConnectionManager<PgConnection>>;
    type PostgresPooled = PooledConnection<ConnectionManager<PgConnection>>;
    type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;
    type SqlitePooled = PooledConnection<ConnectionManager<SqliteConnection>>;

    pub struct Setup {
        pub from: ConnectionPool,
        pub to: ConnectionPool,
        pub image: Container<Postgres>,
        pub temp_file: NamedTempFile,
    }

    fn create_fake_db_sqlite() -> Result<(SqlitePool, NamedTempFile)> {
        let tmp_file = NamedTempFile::new()?;
        let path = tmp_file.path().to_str().unwrap();
        println!("Path: {path}");

        let db_sqlite = ConnectionManager::<SqliteConnection>::new(path);
        let pool = Pool::builder().build(db_sqlite)?;
        pool.get()?
            .run_pending_migrations(MIGRATIONS_SQLITE)
            .map_err(|e| e.to_string())?;
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
            .run_pending_migrations(MIGRATIONS_POSTGRES)
            .map_err(|e| e.to_string())?;

        Ok(Setup {
            from: ConnectionPool::Sqlite(pool),
            to: ConnectionPool::Postgres(pool_postgres),
            image,
            temp_file: tmp_file,
        })
    }

    fn is_exist_table(name: &str, conn: &mut SqlitePooled) {
        let specific_table_check = diesel::sql_query(format!(
            "SELECT 1 FROM sqlite_master WHERE type='table' AND name='{name}'"
        ))
        .execute(conn);

        assert!(
            specific_table_check.is_ok(),
            "La tabella specifica dovrebbe esistere"
        );
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

        is_exist_table("edificio", &mut conn_sq);

        let edificio = NewEdificio {
            chiave: "785461".into(),
            fascicolo: 78591,
            indirizzo: "Via Roma 123".into(),
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

        is_exist_table("stanza", &mut conn_sq);

        let insert_stanza = NewStanza {
            edificio_id: "7878788".into(),
            piano: "T".into(),
            id_spazio: "12587".into(),
            cod_stanza: "001".into(),
            destinazione_uso: "Ufficio".into(),
        };

        diesel::insert_into(stanza::table)
            .values(&insert_stanza)
            .execute(&mut conn_sq)?;

        DatabaseMigrator::migrate_stanza(&mut conn_sq, &mut conn_pg)?;

        let results: Vec<Stanza> = stanza::table.load(&mut conn_pg)?;
        println!("Dopo: {results:#?}");
        Ok(())
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
        //diesel::sql_query("SET session_replication_role = replica").execute(&mut
        // conn_pg)?;

        is_exist_table("infisso", &mut conn_sq);

        let infisso = NewInfisso {
            id: "A".into(),
            edificio_id: "785461".into(),
            tipo: "Porta".into(),
            altezza: 120,
            larghezza: 150,
            materiale: "Legno".into(),
            vetro: "Singolo".into(),
        };

        diesel::insert_into(infisso::table)
            .values(&infisso)
            .execute(&mut conn_sq)?;

        diesel::insert_into(edificio::table)
            .values(&NewEdificio {
                chiave: infisso.edificio_id,
                fascicolo: 78591,
                indirizzo: "Via Roma 123".into(),
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

        is_exist_table("stanza_con_infissi", &mut conn_sq1);
        is_exist_table("stanza_con_infissi", &mut conn_sq2);

        let value_insert = [
            StanzaConInfissi {
                infisso_id: "A".into(),
                edificio_id: "7878788".into(),
                stanza_id: 7,
                num_infisso: 25,
            },
            StanzaConInfissi {
                infisso_id: "B".into(),
                edificio_id: "7878788".into(),
                stanza_id: 8,
                num_infisso: 12,
            },
        ];

        diesel::insert_into(stanza_con_infissi::table)
            .values(&value_insert)
            .execute(&mut conn_sq1)?;

        diesel::insert_into(stanza_con_infissi::table)
            .values(&value_insert)
            .execute(&mut conn_sq2)?;

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

        is_exist_table("utenze", &mut conn_sq);

        diesel::sql_query("INSERT INTO UTENZE (EDIFICIO_ID, TIPO, COD_CONTATORE, INDIRIZZO_CONTATORE) VALUES ($1, $2, $3, $4)")
            .bind::<Text, _>("454545")
            .bind::<Text, _>("acqua")
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

        is_exist_table("fotovoltaico", &mut conn_sq);

        let insert_fotovoltaico = NewFotovoltaico {
            edificio_id: "454545".into(),
            potenza: 30.9,
            proprietario: "Ugo Ugolini".into(),
        };

        diesel::insert_into(fotovoltaico::table)
            .values(&insert_fotovoltaico)
            .execute(&mut conn_sq)?;

        DatabaseMigrator::migrate_fotovoltaico(&mut conn_sq, &mut conn_pg)?;

        let results: Vec<Fotovoltaico> = fotovoltaico::table.load(&mut conn_pg)?;
        println!("Dopo: {results:#?}");

        Ok(())
    }
}
