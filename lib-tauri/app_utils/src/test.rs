use app_interface::database_interface::DatabaseManager;
use app_interface::database_interface::{DatabaseConnector, PostgresPool};
use async_trait::async_trait;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use serde::de::DeserializeOwned;
use std::any::Any;
use std::fs::File;
use std::marker::PhantomData;
use std::{
    error::Error,
    sync::{Arc, Once, OnceLock},
    time::Duration,
};
use tauri::test::{mock_app, MockRuntime};
use tauri::{App, Manager, State};
use testcontainers::ContainerAsync;
use testcontainers_modules::postgres::Postgres;
use tokio::sync::{OnceCell, RwLock};

static CLEANUP_REGISTERED: Once = Once::new();

static POSTGRES_CONTAINER: OnceCell<ContainerAsync<Postgres>> = OnceCell::const_new();

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../app_models/migrations/postgres");

extern "C" fn cleanup() {
    // creo un nuovo thread
    std::thread::spawn(|| {
        // creo un nuovo Tokio runtime, locale al nuovo thread
        let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
        // eseguo codice async in modo sincrono
        rt.block_on(async {
            if let Some(container) = POSTGRES_CONTAINER.get() {
                container
                    .stop()
                    .await
                    .expect("Could not stop postgres container");
            }
        });
    })
        .join() // aspetta che il thread finisca prima di ritornare
        .expect("Cleanup thread panicked");
}

pub async fn get_postgres_container() -> &'static ContainerAsync<Postgres> {
    CLEANUP_REGISTERED.call_once(|| unsafe {
        libc::atexit(cleanup);
    });

    POSTGRES_CONTAINER
        .get_or_init(|| async {
            testcontainers::runners::AsyncRunner::start(Postgres::default())
                .await
                .expect("Failed to start postgres container")
        })
        .await
}

pub async fn get_connection_string() -> String {
    let container = get_postgres_container().await;
    format!(
        "postgres://postgres:postgres@127.0.0.1:{}/postgres",
        &container
            .get_host_port_ipv4(5432)
            .await
            .expect("Docker container not running")
    )
}

pub type Result<T = ()> = std::result::Result<T, Box<dyn Error>>;

// Mock del connector
#[derive(Clone)]
pub struct MockDatabaseConnector {
    pub postgres_should_fail: bool,
}

#[async_trait]
impl DatabaseConnector for MockDatabaseConnector {
    async fn create_postgres_pool(&self) -> PostgresPool {
        if self.postgres_should_fail {
            panic!("Postgres should fail");
        }
        // Crea un pool di test (nota: richiede un DB PostgreSQL di test)
        let manager = ConnectionManager::<PgConnection>::new(get_connection_string().await);
        let pool = Pool::builder()
            .max_size(1)
            .connection_timeout(Duration::from_secs(1))
            .build(manager)
            .expect("Failed to create pool");

        {
            let mut conn = pool.get().unwrap();
            conn.run_pending_migrations(MIGRATIONS).unwrap();
        }

        pool
    }

    //fn as_any(&self) -> &dyn std::any::Any {
    //     self
    // }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub async fn create_test_postgres_container() -> ContainerAsync<Postgres> {
    testcontainers::runners::AsyncRunner::start(Postgres::default())
        .await
        .expect("Failed to start postgres container")
}

pub struct IsolatedTestDatabaseConnector {
    pub postgres_should_fail: bool,
    pub container: Arc<RwLock<Option<ContainerAsync<Postgres>>>>,
}

impl IsolatedTestDatabaseConnector {
    pub async fn new() -> Self {
        let container = create_test_postgres_container().await;

        Self {
            postgres_should_fail: false,
            container: Arc::new(RwLock::new(Some(container))),
        }
    }

    pub async fn cleanup(&self) -> Result<()> {
        let mut container = self.container.write().await;
        if let Some(container_instance) = container.take() {
            container_instance.stop().await?;
            println!("Container stopped successfully");
        }
        Ok(())
    }
}

#[async_trait]
impl DatabaseConnector for IsolatedTestDatabaseConnector {
    async fn create_postgres_pool(&self) -> PostgresPool {
        if self.postgres_should_fail {
            panic!("Postgres should fail");
        }

        let container = self.container.read().await;
        let connection_string = format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            container
                .as_ref()
                .unwrap()
                .get_host_port_ipv4(5432)
                .await
                .expect("Docker container not running")
        );

        let manager = ConnectionManager::<PgConnection>::new(connection_string);
        let pool = Pool::builder()
            .max_size(1)
            .connection_timeout(Duration::from_secs(30))
            .build(manager)
            .expect("Failed to create pool");

        {
            let mut conn = pool.get().unwrap();
            conn.run_pending_migrations(MIGRATIONS).unwrap();
        }

        pool
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Clone for Box<IsolatedTestDatabaseConnector> {
    fn clone(&self) -> Self {
        Box::new(IsolatedTestDatabaseConnector {
            postgres_should_fail: self.postgres_should_fail,
            container: self.container.clone(),
        })
    }
}

static POOL_DB: OnceLock<Pool<ConnectionManager<PgConnection>>> = OnceLock::new();

pub async fn create_postgres_pool() -> &'static Pool<ConnectionManager<PgConnection>> {
    POOL_DB.get_or_init(|| {
        let connection_string =
            "postgres://app_user:app_password@127.0.0.1:5432/app_development".to_string();
        let manager = ConnectionManager::<PgConnection>::new(connection_string);
        Pool::builder()
            .max_size(1)
            .connection_timeout(Duration::from_secs(1))
            .build(manager)
            .expect("Failed to create pool")
    })
}

/// A structure representing the test environment used for integration or unit tests.
///
/// This struct encapsulates the components required for setting up and running
/// isolated tests, such as a database connector and an application instance with
/// a mock runtime.
///
/// # Fields
///
/// * `connector` - A boxed instance of `IsolatedTestDatabaseConnector`, which provides
///   an isolated database connection for running tests without affecting the primary
///   database or other tests.
///
/// * `app` - An instance of `App` configured to use a `MockRuntime`. This represents
///   the application under test, allowing mock interactions or controlled runtime behavior.
///
/// # Usage
///
/// The `TestEnvironment` is typically used to initialize and configure the test setup,
/// ensuring a consistent and reliable testing framework.
///
/// ```
pub struct TestServiceEnvironment<D> {
    connector: Box<IsolatedTestDatabaseConnector>,
    app: App<MockRuntime>,
    phantom_data: PhantomData<D>,
}

impl<D> TestServiceEnvironment<D>
where
    D: DatabaseManager + Send + Sync + 'static,
{
    pub async fn new<T, F>(insert_data: T) -> Result<Self>
    where
        T: Fn(D) -> F + Send + 'static,
        F: std::future::Future<Output=Result<()>> + Send + 'static,
        D: Clone,
    {
        let connector = Box::new(IsolatedTestDatabaseConnector::new().await);
        let db_manager = D::with_connector(connector.clone()).await;

        insert_data(db_manager.clone()).await?;

        let app = mock_app();
        app.manage(db_manager);

        Ok(TestServiceEnvironment {
            app,
            connector,
            phantom_data: Default::default(),
        })
    }

    pub fn database(&self) -> State<'_, D> {
        self.app.state::<D>()
    }
}

impl<D> Drop for TestServiceEnvironment<D> {
    fn drop(&mut self) {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                if let Err(e) = self.connector.cleanup().await {
                    eprintln!("Errore durante cleanup: {}", e);
                }
            });
        });
    }
}

pub fn read_json_file<T: DeserializeOwned>(
    file_path: &str,
) -> std::result::Result<Vec<T>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let content = std::io::read_to_string(file)?;

    match serde_json::from_str(&content) {
        Ok(data) => Ok(data),
        Err(e) => Err(Box::new(e)),
    }
}
