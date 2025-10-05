pub mod utils;

use std::{
    error::Error,
    marker::PhantomData,
    sync::{
        Arc, Once, OnceLock,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use app_interface::{
    dao_interface::{DAO as DAOTrait, crud_operations::Insert},
    database_interface::{DatabaseManagerTrait, PostgresPooled},
};
use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use serde::de::DeserializeOwned;
use tauri::{
    AppHandle, Manager, State,
    test::{MockRuntime, mock_app},
};
use testcontainers::ContainerAsync;
use testcontainers_modules::postgres::Postgres;
use tokio::sync::OnceCell;

use crate::{
    path_data_fake,
    test::{impl_database_connector::IsolatedTestDatabaseConnector, utils::read_json_file},
};

pub type ResultTest<T = ()> = Result<T, Box<dyn Error + Send + Sync>>;

static CLEANUP_REGISTERED: Once = Once::new();

static POSTGRES_CONTAINER: OnceCell<ContainerAsync<Postgres>> = OnceCell::const_new();

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../app_models/migrations/");

pub mod impl_database_connector {
    use std::{any::Any, sync::Arc, time::Duration};

    use app_interface::database_interface::{DatabaseConnector, PostgresPool};
    use async_trait::async_trait;
    use diesel::{
        PgConnection,
        r2d2::{ConnectionManager, Pool},
    };
    use diesel_migrations::MigrationHarness;
    use testcontainers::ContainerAsync;
    use testcontainers_modules::postgres::Postgres;
    use tokio::sync::RwLock;

    use crate::test::{MIGRATIONS, create_test_postgres_container, get_connection_string};

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

        fn as_any_mut(&mut self) -> &mut dyn Any { self }
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

        pub async fn cleanup(&self) -> crate::test::ResultTest<()> {
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

        fn as_any_mut(&mut self) -> &mut dyn Any { self }
    }

    impl Clone for Box<IsolatedTestDatabaseConnector> {
        fn clone(&self) -> Self {
            Box::new(IsolatedTestDatabaseConnector {
                postgres_should_fail: self.postgres_should_fail,
                container: self.container.clone(),
            })
        }
    }
}

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
                println!("postgres container is close");
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

pub async fn create_test_postgres_container() -> ContainerAsync<Postgres> {
    testcontainers::runners::AsyncRunner::start(Postgres::default())
        .await
        .expect("Failed to start postgres container")
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

/// A structure representing the test environment used for integration or unit
/// tests.
///
/// This struct encapsulates the components required for setting up and running
/// isolated tests, such as a database connector and an application instance
/// with a mock runtime.
///
/// # Fields
///
/// * `connector` - A boxed instance of `IsolatedTestDatabaseConnector`, which
///   provides an isolated database connection for running tests without
///   affecting the primary database or other tests.
///
/// * `app` - An instance of `App` configured to use a `MockRuntime`. This
///   represents the application under test, allowing mock interactions or
///   controlled runtime behavior.
///
/// # Usage
///
/// The `TestEnvironment` is typically used to initialize and configure the test
/// setup, ensuring a consistent and reliable testing framework.
///
/// ```
pub struct TestServiceEnvironment<D> {
    connector: Box<IsolatedTestDatabaseConnector>,
    app: Arc<AppHandle<MockRuntime>>,
    cleanup_done: Arc<AtomicBool>,
    phantom_data: PhantomData<D>,
}

impl<D> TestServiceEnvironment<D>
where
    D: DatabaseManagerTrait + Send + Sync + 'static,
{
    pub async fn new<T, F>(insert_data: T) -> ResultTest<Self>
    where
        T: Fn(D) -> F + Send + 'static,
        F: std::future::Future<Output = ResultTest<()>> + Send + 'static,
        D: Clone,
    {
        let connector = Box::new(IsolatedTestDatabaseConnector::new().await);
        let db_manager = D::with_connector(connector.clone()).await;

        insert_data(db_manager.clone()).await?;

        let app = mock_app();
        app.manage(db_manager);

        let cleanup_done = Arc::new(AtomicBool::new(false));
        let connector_clone = connector.clone();
        let cleanup_flag = cleanup_done.clone();

        let original_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            if !cleanup_flag.load(Ordering::Acquire) {
                eprintln!("Panic revealed, cleaning up...");
                tokio::task::block_in_place(|| {
                    tokio::runtime::Handle::current().block_on(async {
                        if let Err(e) = connector_clone.cleanup().await {
                            eprintln!("Errore durante cleanup: {}", e);
                        }
                    });
                });
                cleanup_flag.store(true, Ordering::Relaxed);
            }
            original_hook(panic_info);
        }));

        Ok(TestServiceEnvironment {
            app: Arc::new(app.handle().clone()),
            connector,
            cleanup_done,
            phantom_data: Default::default(),
        })
    }

    pub fn app(&self) -> Arc<AppHandle<MockRuntime>> { self.app.clone() }

    pub fn database(&self) -> State<'_, D> { self.app.state::<D>() }

    pub fn set_state_app<T>(&self, state: T) -> bool
    where
        T: Send + Sync + 'static,
    {
        self.app.manage(state)
    }

    pub fn state_app<T>(&self) -> State<'_, T>
    where
        T: Send + Sync + 'static,
    {
        self.app.state::<T>()
    }

    pub async fn cleanup(&self) -> ResultTest {
        if !self.cleanup_done.load(Ordering::Acquire) {
            self.connector.cleanup().await?;
            self.cleanup_done.store(true, Ordering::Release);
            println!("✅ Cleanup manuale completato");
        }
        Ok(())
    }
}

impl<D> Drop for TestServiceEnvironment<D> {
    fn drop(&mut self) {
        if !self.cleanup_done.load(Ordering::Acquire) {
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    if let Err(e) = self.connector.cleanup().await {
                        eprintln!("❌ Errore during cleanup in Drop: {}", e);
                    } else {
                        println!("🧹 Cleanup automatic completely in Drop");
                    }
                });
            });
            self.cleanup_done.store(true, Ordering::Release);
        }
    }
}

pub struct TestDaoEnvironment {
    pool: Pool<ConnectionManager<PgConnection>>,
    container: Arc<ContainerAsync<Postgres>>,
    container_close_done: Arc<AtomicBool>,
}

impl TestDaoEnvironment {
    pub async fn new() -> ResultTest<Self> {
        let container = Arc::new(create_test_postgres_container().await);

        let connection_string = format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            &container
                .get_host_port_ipv4(5432)
                .await
                .expect("Docker container not running")
        );
        let manager = ConnectionManager::<PgConnection>::new(connection_string);
        let pool = Pool::builder()
            .max_size(1)
            .connection_timeout(Duration::from_secs(1))
            .build(manager)
            .expect("Failed to create pool");

        {
            let mut conn = pool.get()?;
            conn.run_pending_migrations(MIGRATIONS)?;
        }

        let container_close_done = Arc::new(AtomicBool::new(false));
        let cleanup_flag = container_close_done.clone();
        let container_clone = container.clone();

        let original_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            if !cleanup_flag.load(Ordering::Acquire) {
                eprintln!("Panic revealed, cleaning up...");
                tokio::task::block_in_place(|| {
                    tokio::runtime::Handle::current().block_on(async {
                        if let Err(e) = container_clone.stop().await {
                            eprintln!("Errore durante cleanup: {}", e);
                        }
                    });
                });
                cleanup_flag.store(true, Ordering::Relaxed);
            }
            original_hook(panic_info);
        }));

        Ok(Self {
            container,
            pool,
            container_close_done,
        })
    }

    pub fn insert_data<DAO, DTO, Model>(&self, data_file_name: &str) -> ResultTest
    where
        DTO: DeserializeOwned,
        DAO: DAOTrait + Insert<Model> + 'static,
        Model: From<DTO>,
    {
        let mut conn = self.get_pooled_connection()?;

        let dto_data = read_json_file::<DTO>(path_data_fake!(data_file_name).as_ref())?;
        for dto_item in dto_data {
            match <DAO as Insert<Model>>::insert(&mut conn, dto_item.into()) {
                Ok(_) => {}
                Err(e) => return Err(Box::new(e)),
            }
        }
        Ok(())
    }

    pub fn get_pooled_connection(&self) -> ResultTest<PostgresPooled> { Ok(self.pool.get()?) }

    pub async fn cleanup(&self) -> ResultTest {
        if !self.container_close_done.load(Ordering::Acquire) {
            self.container.stop().await?;
            self.container_close_done.store(true, Ordering::Release);
            println!("✅ Cleanup manuale completato");
        }
        Ok(())
    }
}

impl Drop for TestDaoEnvironment {
    fn drop(&mut self) {
        if !self.container_close_done.load(Ordering::Acquire) {
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    if let Err(e) = self.container.stop().await {
                        eprintln!("Errore durante cleanup: {}", e);
                    } else {
                        println!("Container stopped successfully");
                    }
                });
            });
            self.container_close_done.store(true, Ordering::Release);
        }
    }
}
