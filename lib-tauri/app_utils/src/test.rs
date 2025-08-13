use app_interface::database_interface::{DatabaseConnector, PostgresPool};
use async_trait::async_trait;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::error::Error;
use std::sync::{Once, OnceLock};
use std::time::Duration;
use testcontainers::ContainerAsync;
use testcontainers_modules::postgres::Postgres;
use tokio::sync::OnceCell;

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

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
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