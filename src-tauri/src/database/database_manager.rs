use crate::database::error::DbError;
use crate::database::DatabaseMigrator;
use async_trait::async_trait;
use diesel::{
    prelude::SqliteConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};
use dotenvy::dotenv;
use log::info;
use std::{
    env,
    fmt::{Debug, Formatter},
    ops::Deref,
    sync::Arc,
};
use tokio::sync::Mutex;

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;
pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

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

pub enum DatabaseConnection {
    Sqlite(PooledConnection<ConnectionManager<SqliteConnection>>),
    Postgres(PooledConnection<ConnectionManager<PgConnection>>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum DatabaseType {
    Sqlite,
    Postgres,
}

#[async_trait]
pub trait DatabaseConnector {
    async fn try_postgres_connection(&self) -> Result<PostgresPool, DbError>;
    fn create_sqlite_pool(&self, path: Option<&str>) -> Result<SqlitePool, DbError>;
}

pub struct RealDatabaseConnector;

#[async_trait]
impl DatabaseConnector for RealDatabaseConnector {
    async fn try_postgres_connection(&self) -> Result<PostgresPool, DbError> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")?;

        println!("Database URL: {}", database_url);

        let pool_size = env::var("DATABASE_POOL_SIZE")
            .unwrap_or_else(|_| "10".to_string())
            .parse::<u32>()
            .expect("DATABASE_POOL_SIZE must be a number");

        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .max_size(pool_size)
            .build(manager)
            .map_err(|e| DbError::ConnectionPoolError(e.to_string()))?;

        // test per verificare che il pool funziona correttamente
        tokio::time::timeout(std::time::Duration::from_secs(2), async {
            let _conn = pool
                .get()
                .map_err(|e| DbError::ConnectionPoolError(e.to_string()))?;
            Ok::<(), DbError>(())
        })
        .await??;

        Ok(pool)
    }

    fn create_sqlite_pool(&self, path: Option<&str>) -> Result<SqlitePool, DbError> {
        let path = path.unwrap_or(":memory:");

        let pool = ConnectionManager::<SqliteConnection>::new(path);
        let pool = Pool::builder()
            .max_size(10)
            .build(pool)
            .map_err(|e| DbError::ConnectionPoolError(e.to_string()))?;

        // test per verificare che il pool funziona correttamente
        let _conn = pool
            .get()
            .map_err(|e| DbError::ConnectionPoolError(e.to_string()))?;

        Ok(pool)
    }
}

type ConnectorDatabase = Box<dyn DatabaseConnector + Send + Sync>;

/// A struct that manages database connections and operations in a thread-safe manner.
///
/// The `DatabaseManager` encapsulates the necessary components to manage a database connection pool
/// and interact with a specific type of database using an appropriate connector. It ensures thread safety
/// by using `Arc` and `Mutex` where shared access is needed.
///
pub struct DatabaseManager {
    /// Manages a pool of database connections.
    pool: Arc<Mutex<ConnectionPool>>,
    /// Type of database
    db_type: Arc<Mutex<DatabaseType>>,
    /// Defines how to interact with the underlying database
    connector: ConnectorDatabase,
}

impl DatabaseManager {
    pub async fn new() -> Self {
        Self::with_connector(Box::new(RealDatabaseConnector)).await
    }

    pub async fn with_connector(connector: ConnectorDatabase) -> Self {
        let (initial_pool, initial_type) = match connector.try_postgres_connection().await {
            Ok(pg_pool) => {
                log::info!("Start with Postgres");
                (ConnectionPool::Postgres(pg_pool), DatabaseType::Postgres)
            }
            Err(e) => {
                log::warn!("Failed to connect to Postgres: {e}");
                log::info!("Start with SQLite (in memory)");
                let sqlite_pool = connector.create_sqlite_pool(None).unwrap();
                (ConnectionPool::Sqlite(sqlite_pool), DatabaseType::Sqlite)
            }
        };

        Self {
            pool: Arc::new(Mutex::new(initial_pool)),
            db_type: Arc::new(Mutex::new(initial_type)),
            connector,
        }
    }

    pub async fn get_connection(&self) -> Result<DatabaseConnection, DbError> {
        let pool_lock = self.pool.lock().await;

        match &*pool_lock {
            ConnectionPool::Sqlite(pool) => {
                let conn = pool
                    .get()
                    .map_err(|e| DbError::ConnectionPoolError(e.to_string()))?;
                Ok(DatabaseConnection::Sqlite(conn))
            }
            ConnectionPool::Postgres(pool) => {
                let conn = pool
                    .get()
                    .map_err(|e| DbError::ConnectionPoolError(e.to_string()))?;
                Ok(DatabaseConnection::Postgres(conn))
            }
        }
    }

    pub async fn get_db_type(&self) -> DatabaseType {
        let type_lock = self.db_type.lock().await;
        type_lock.deref().clone()
    }

    pub async fn switch_to_postgres(&self) -> Result<(), DbError> {
        let mut pool_lock = self.pool.lock().await;
        let mut type_lock = self.db_type.lock().await;

        if matches!(type_lock.deref(), DatabaseType::Postgres) {
            log::info!("Just connected to Postgres");
            return Ok(());
        }

        match self.connector.try_postgres_connection().await {
            Ok(pg_pool) => {
                log::info!("Switching to Postgres");
                let new_pool = ConnectionPool::Postgres(pg_pool);

                self.migrate_data_sqlite_to_postgres(&pool_lock, &new_pool)
                    .await?;

                *pool_lock = new_pool;
                *type_lock = DatabaseType::Postgres;

                Ok(())
            }
            Err(e) => {
                log::warn!("Failed to connect to Postgres: {e}");
                Err(e)
            }
        }
    }
    pub async fn switch_to_sqlite(&self, path: &str) -> Result<(), DbError> {
        let mut pool_lock = self.pool.lock().await;
        let mut type_lock = self.db_type.lock().await;

        if matches!(type_lock.deref(), DatabaseType::Postgres) {
            log::info!("Just connected to Postgres");
            return Ok(());
        }

        match self.connector.create_sqlite_pool(Some(path)) {
            Ok(sqlite_pool) => {
                log::info!("Switching to Postgres");

                *pool_lock = ConnectionPool::Sqlite(sqlite_pool);
                *type_lock = DatabaseType::Sqlite;

                Ok(())
            }
            Err(e) => {
                log::warn!("Failed to connect to Sqlite: {e}");
                Err(e)
            }
        }
    }

    async fn migrate_data_sqlite_to_postgres(
        &self,
        sqlite_pool: &ConnectionPool,
        pg_pool: &ConnectionPool,
    ) -> Result<(), DbError> {
        info!("Migrating data from SQLite to Postgres");
        let database_migrator = DatabaseMigrator::new(sqlite_pool, pg_pool);
        database_migrator.migrate().map_err(|e| {
            log::error!("Migration failed: {e}");
            e
        })?;
        info!("Migration completed");
        Ok(())
    }
}

impl Clone for DatabaseManager {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
            db_type: self.db_type.clone(),
            connector: Box::new(RealDatabaseConnector),
        }
    }
}

impl Debug for DatabaseManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DatabaseManager")
            .field("pool", &self.pool)
            .field("db_type", &self.db_type)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock del connector
    #[derive(Clone)]
    pub struct MockDatabaseConnector {
        postgres_should_fail: bool,
        sqlite_should_fail: bool,
    }

    #[async_trait]
    impl DatabaseConnector for MockDatabaseConnector {
        async fn try_postgres_connection(
            &self,
        ) -> Result<Pool<ConnectionManager<PgConnection>>, DbError> {
            if self.postgres_should_fail {
                return Err(DbError::ConnectionTimeout);
            }

            // Crea un pool di test (nota: richiede un DB PostgreSQL di test)
            let database_url = "postgres://app_user:app_password@127.0.0.1/app_development";
            let manager = ConnectionManager::<PgConnection>::new(database_url);
            Pool::builder()
                .max_size(1)
                .build(manager)
                .map_err(|e| DbError::ConnectionPoolError(e.to_string()))
        }

        fn create_sqlite_pool(
            &self,
            path: Option<&str>,
        ) -> Result<Pool<ConnectionManager<SqliteConnection>>, DbError> {
            if self.sqlite_should_fail {
                return Err(DbError::ConnectionPoolError("Test error".into()));
            }

            let path = path.unwrap_or(":memory:");

            let manager = ConnectionManager::<SqliteConnection>::new(path);
            Pool::builder()
                .max_size(1)
                .build(manager)
                .map_err(|e| DbError::ConnectionPoolError(e.to_string()))
        }
    }

    #[tokio::test]
    async fn test_new_with_postgres_success() {
        let mock_connector = MockDatabaseConnector {
            postgres_should_fail: false,
            sqlite_should_fail: false,
        };

        let db_manager = DatabaseManager::with_connector(Box::new(mock_connector)).await;
        let db_type = db_manager.get_db_type().await;

        assert!(matches!(db_type, DatabaseType::Postgres));
    }

    #[tokio::test]
    async fn test_new_fallback_to_sqlite() {
        let mock_connector = MockDatabaseConnector {
            postgres_should_fail: true,
            sqlite_should_fail: false,
        };

        let db_manager = DatabaseManager::with_connector(Box::new(mock_connector)).await;
        let db_type = db_manager.get_db_type().await;

        assert!(matches!(db_type, DatabaseType::Sqlite));
    }

    #[tokio::test]
    #[should_panic]
    async fn test_new_both_fail() {
        let mock_connector = MockDatabaseConnector {
            postgres_should_fail: true,
            sqlite_should_fail: true,
        };

        let _db_manager = DatabaseManager::with_connector(Box::new(mock_connector)).await;
    }
}
