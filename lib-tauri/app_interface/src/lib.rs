#[cfg(feature = "database")]
pub mod database_interface {
    use app_error::database_error::DbError;
    use async_trait::async_trait;
    use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
    use diesel::PgConnection;
    use std::any::Any;

    pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;
    pub type PostgresPooled = PooledConnection<ConnectionManager<PgConnection>>;

    #[async_trait]
    pub trait DatabaseConnector {
        async fn create_postgres_pool(&self) -> PostgresPool;
        fn as_any_mut(&mut self) -> &mut dyn Any;
    }

    #[async_trait]
    pub trait DatabaseManager {
        async fn get_connection(&self) -> Result<PostgresPooled, DbError>;
    }
}

#[cfg(feature = "dao")]
pub mod dao_interface {
    pub trait DAO {
        type Item;
    }

    pub mod crud_operations {
        use crate::dao_interface::DAO;
        use crate::database_interface::PostgresPooled;
        use app_error::DomainError;

        pub trait Get<T, K>: DAO {
            fn get(conn: &mut PostgresPooled, id: K) -> Result<Self::Item, DomainError>;
        }

        pub trait GetAll<T>: DAO {
            fn get_all(conn: &mut PostgresPooled) -> Result<Vec<Self::Item>, DomainError>;
        }

        pub trait Insert<T>: DAO {
            fn insert(conn: &mut PostgresPooled, item: T) -> Result<Self::Item, DomainError>;
        }

        pub trait Update<T, Id>: DAO {
            fn update(conn: &mut PostgresPooled, id: Id, item: T) -> Result<Self::Item, DomainError>;
        }

        #[allow(dead_code)]
        pub trait Delete<T, K>: DAO {
            fn delete(conn: &mut PostgresPooled, item: K) -> Result<bool, DomainError>;
        }
    }
}

#[cfg(feature = "dto")]
pub mod dto_interface {
    pub trait DTO {}
}

#[cfg(feature = "services")]
pub mod service_interface {
    use crate::database_interface::DatabaseManager;
    use crate::dto_interface::DTO;
    use app_error::AppResult;
    use async_trait::async_trait;
    use tauri::State;

    #[async_trait]
    pub trait CreateService<T>
    where
        T: DTO,
    {
        async fn create(db: State<'_, impl DatabaseManager + Send + Sync>, item: T) -> AppResult<T>;
    }

    #[allow(dead_code)]
    #[async_trait]
    pub trait RetrieveOneService<T, K>
    where
        T: DTO,
    {
        async fn retrieve_one(db: State<'_, impl DatabaseManager + Send + Sync>, id: K) -> AppResult<T>;
    }

    #[async_trait]
    pub trait RetrieveManyService<T>
    where
        T: DTO,
    {
        async fn retrieve_many(db: State<'_, impl DatabaseManager + Send + Sync>) -> AppResult<Vec<T>>;
    }

    #[async_trait]
    pub trait UpdateService<T>
    where
        T: DTO,
    {
        async fn update(db: State<'_, impl DatabaseManager + Send + Sync>, item: T) -> AppResult<T>;
    }

    #[allow(dead_code)]
    #[async_trait]
    pub trait DeleteService<T, K>
    where
        T: DTO,
    {
        async fn delete(db: State<'_, impl DatabaseManager + Send + Sync>, id: K) -> AppResult<bool>;
    }

}