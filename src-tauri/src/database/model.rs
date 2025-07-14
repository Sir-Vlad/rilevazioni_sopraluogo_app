use diesel::r2d2::{self, ConnectionManager};
use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use serde::Serialize;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub struct Database {
    pool: Option<DbPool>,
}

impl Database {
    pub fn new() -> Self {
        Self { pool: None }
    }

    pub fn init(&mut self) -> Result<(), diesel::r2d2::Error> {
        dotenv().ok();

        let database_url =
            env::var("DATABASE_URL").expect("DATABASE_URL must setting in the file .env");

        let pool_size = env::var("DATABASE_POOL_SIZE")
            .unwrap_or_else(|_| "10".to_string())
            .parse::<u32>()
            .expect("DATABASE_POOL_SIZE must be a number");

        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = DbPool::builder()
            .max_size(pool_size)
            .build(manager)
            .expect("Failed to create pool.");
        self.pool = Some(pool);
        Ok(())
    }

    pub fn get_conn(&self) -> Result<DbConnection, diesel::result::Error> {
        match &self.pool {
            Some(pool) => pool.get().map_err(|_| {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UnableToSendCommand,
                    Box::new("Failed to get connection from pool".to_string()),
                )
            }),
            None => Err(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new("Database not initialized".to_string()),
            )),
        }
    }

    pub fn with_transaction<F, T>(&self, op: F) -> Result<T, diesel::result::Error>
    where
        T: for<'de> serde::Deserialize<'de> + serde::Serialize,
        F: FnOnce(&mut PgConnection) -> Result<T, diesel::result::Error>,
    {
        let mut conn = self.get_conn()?;
        conn.transaction(|conn| op(conn))
    }
}

#[derive(Serialize, Clone)]
pub struct DatabaseEventPayload {
    pub(crate) type_event: &'static str,
    pub(crate) path: String,
}
