use crate::database::database_manager::PostgresPool;
use crate::database::DatabaseConnector;
use async_trait::async_trait;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use std::error::Error;
use std::sync::Once;
use std::time::Duration;
use testcontainers::ContainerAsync;
use testcontainers_modules::postgres::Postgres;
use tokio::sync::OnceCell;

static CLEANUP_REGISTERED: Once = Once::new();

static POSTGRES_CONTAINER: OnceCell<ContainerAsync<Postgres>> = OnceCell::const_new();

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

// pub struct Setup {
//     pub from: ConnectionPool,
//     pub to: ConnectionPool,
//     pub image: Container<Postgres>,
//     pub temp_file: NamedTempFile,
// }

// pub fn setup() -> Result<Setup> {
//     let (pool, tmp_file) = create_fake_db_sqlite()?;
//
//     let image = testcontainers::runners::SyncRunner::start(Postgres::default())?;
//
//     let connection_string = &format!(
//         "postgres://postgres:postgres@127.0.0.1:{}/postgres",
//         image.get_host_port_ipv4(5432)?
//     );
//     let db_postgres = ConnectionManager::<PgConnection>::new(connection_string);
//     let pool_postgres = Pool::builder().build(db_postgres)?;
//     pool_postgres
//         .get()?
//         .run_pending_migrations(MIGRATIONS_POSTGRES)
//         .map_err(|e| e.to_string())?;
//
//     Ok(Setup {
//         from: ConnectionPool::Sqlite(pool),
//         to: ConnectionPool::Postgres(pool_postgres),
//         image,
//         temp_file: tmp_file,
//     })
// }

// Mock del connector
#[derive(Clone)]
pub struct MockDatabaseConnector {
    pub(crate) postgres_should_fail: bool,
}

#[async_trait]
impl DatabaseConnector for MockDatabaseConnector {
    async fn create_postgres_pool(&self) -> PostgresPool {
        if self.postgres_should_fail {
            panic!("Postgres should fail");
        }

        // Crea un pool di test (nota: richiede un DB PostgreSQL di test)
        let container = get_postgres_container().await;
        let connection_string = &format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            &container
                .get_host_port_ipv4(5432)
                .await
                .expect("Docker container not running")
        );
        let manager = ConnectionManager::<PgConnection>::new(connection_string);
        Pool::builder()
            .max_size(1)
            .connection_timeout(Duration::from_secs(1))
            .build(manager)
            .expect("Failed to create pool")
    }

    //fn as_any(&self) -> &dyn std::any::Any {
    //     self
    // }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
