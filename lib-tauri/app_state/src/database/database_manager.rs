use app_utils::app_error::database_error::DbError;
pub use app_utils::app_interface::database_interface::{
    ConnectorDatabase, DatabaseConnector, DatabaseManager as DatabaseManagerInterface,
    PostgresPool, PostgresPooled,
};
use async_trait::async_trait;
use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use std::{
    any::Any,
    env,
    fmt::{Debug, Formatter},
    path::Path,
    sync::Arc,
};
use tokio::sync::{Mutex, RwLock, RwLockReadGuard};

pub struct RealDatabaseConnector;

#[async_trait]
impl DatabaseConnector for RealDatabaseConnector {
    async fn create_postgres_pool(&self) -> PostgresPool {
        dotenvy::from_path(Path::new("../../.env")).ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool_size = env::var("DATABASE_POOL_SIZE")
            .unwrap_or_else(|_| "10".to_string())
            .parse::<u32>()
            .expect("DATABASE_POOL_SIZE must be a number");

        let manager = ConnectionManager::<PgConnection>::new(database_url);
        Pool::builder()
            .max_size(pool_size)
            .build(manager)
            .expect("Failed to create pool")
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// A struct that manages database connections and operations in a thread-safe manner.
///
/// The `DatabaseManager` encapsulates the necessary components to manage a database connection pool
/// and interact with a specific type of database using an appropriate connector. It ensures thread safety
/// by using `Arc` and `Mutex` where shared access is needed.
///
pub struct DatabaseManager {
    /// Manages a postgres pool of database connections.
    postgres_pool: Arc<Mutex<PostgresPool>>,
    /// Defines how to interact with the underlying database
    connector: RwLock<ConnectorDatabase>,
}

#[async_trait]
impl DatabaseManagerInterface for DatabaseManager {
    async fn with_connector(connector: ConnectorDatabase) -> Self {
        let postgres_pool = connector.create_postgres_pool().await;

        Self {
            postgres_pool: Arc::new(Mutex::new(postgres_pool)),
            connector: RwLock::new(connector),
        }
    }

    async fn get_connection(&self) -> Result<PostgresPooled, DbError> {
        let pool_guard = self.postgres_pool.lock().await;
        let conn = pool_guard
            .get()
            .map_err(|e| DbError::ConnectionPoolError(e.to_string()))?;
        Ok(conn)
    }
}

impl DatabaseManager {
    pub async fn new() -> Self {
        Self::with_connector(Box::new(RealDatabaseConnector)).await
    }

    pub async fn get_connector(&self) -> RwLockReadGuard<'_, ConnectorDatabase> {
        self.connector.read().await
    }

    #[cfg(test)]
    pub async fn modify_connector_field<T, F>(&self, f: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnOnce(&mut T),
        T: DatabaseConnector + Send + Sync + 'static,
    {
        let mut connector = self.connector.write().await;
        if let Some(connector) = connector.as_any_mut().downcast_mut::<T>() {
            f(connector);
            Ok(())
        } else {
            Err(format!(
                "Failed to downcast connector to type: {}",
                std::any::type_name::<T>()
            )
            .into())
        }
    }
}

impl Clone for DatabaseManager {
    fn clone(&self) -> Self {
        Self {
            postgres_pool: self.postgres_pool.clone(),
            connector: RwLock::new(Box::new(RealDatabaseConnector)),
        }
    }
}

impl Debug for DatabaseManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DatabaseManager")
            .field("pool", &self.postgres_pool)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use app_utils::test::{get_postgres_container, impl_database_connector::MockDatabaseConnector};
    use async_trait::async_trait;
    use diesel::r2d2::R2D2Connection;
    use std::any::Any;
    use std::sync::Arc;
    use testcontainers::ContainerAsync;
    use testcontainers_modules::postgres::Postgres;

    async fn create_database_manager(connector: MockDatabaseConnector) -> Arc<DatabaseManager> {
        Arc::new(DatabaseManager::with_connector(Box::new(connector)).await)
    }

    #[tokio::test]
    async fn test_connection_pool() {
        let mock_connector = MockDatabaseConnector {
            postgres_should_fail: false,
        };

        let db_manager = create_database_manager(mock_connector).await;

        assert!(db_manager.get_connection().await.is_ok());
    }

    #[tokio::test]
    #[should_panic]
    async fn test_connection_pool_fails() {
        let mock_connector = MockDatabaseConnector {
            postgres_should_fail: true,
        };

        let _db_manager = create_database_manager(mock_connector).await;
    }

    #[tokio::test]
    async fn test_concurrent_connections() {
        let mock_connector = MockDatabaseConnector {
            postgres_should_fail: false,
        };

        let db_manager = create_database_manager(mock_connector).await;

        let mut handles = vec![];

        // Creare più task che richiedono connessioni contemporaneamente
        for _ in 0..10 {
            let manager_clone = db_manager.clone();
            handles.push(tokio::spawn(async move {
                let conn = manager_clone.get_connection().await;

                assert!(conn.is_ok());
            }));
        }

        // Attendere che tutti i task siano completati
        for handle in handles {
            handle.await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_connection_timeout() {
        // Creare un mock connector che simula un timeout
        struct TimeoutConnector<'a> {
            container_async: &'a ContainerAsync<Postgres>,
        }

        #[async_trait]
        impl DatabaseConnector for TimeoutConnector<'static> {
            async fn create_postgres_pool(&self) -> PostgresPool {
                // Creare un pool con timeout molto breve
                let manager = ConnectionManager::<PgConnection>::new(format!(
                    "postgres://postgres:postgres@127.0.0.1:{}/postgres",
                    self.container_async.get_host_port_ipv4(5432).await.unwrap()
                ));
                Pool::builder()
                    .max_size(1)
                    .connection_timeout(std::time::Duration::from_millis(100))
                    .build(manager)
                    .expect("Failed to create pool")
            }

            // fn as_any(&self) -> &dyn Any {
            //     self
            // }
            //
            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }
        }

        let container = get_postgres_container().await;
        let connector = TimeoutConnector {
            container_async: container,
        };

        let db_manager = DatabaseManager::with_connector(Box::new(connector)).await;

        let mut pooled = db_manager.get_connection().await.unwrap();
        assert!(pooled.ping().is_ok());

        container.stop().await.unwrap();

        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

        assert!(pooled.ping().is_err());
    }
}
