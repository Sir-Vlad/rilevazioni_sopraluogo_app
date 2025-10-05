#[cfg(feature = "database")]
pub mod database_interface {
    use std::any::Any;

    use app_error::database_error::DbError;
    use async_trait::async_trait;
    use diesel::{
        PgConnection,
        r2d2::{ConnectionManager, Pool, PooledConnection},
    };

    pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;
    pub type PostgresPooled = PooledConnection<ConnectionManager<PgConnection>>;

    pub type ConnectorDatabase = Box<dyn DatabaseConnector + Send + Sync>;

    #[async_trait]
    pub trait DatabaseConnector {
        async fn create_postgres_pool(&self) -> PostgresPool;
        fn as_any_mut(&mut self) -> &mut dyn Any;
    }

    #[async_trait]
    pub trait DatabaseManagerTrait {
        async fn with_connector(connector: ConnectorDatabase) -> Self;
        async fn get_connection(&self) -> Result<PostgresPooled, DbError>;
    }
}

#[cfg(feature = "dao")]
pub mod dao_interface {
    pub trait DAO {}

    pub mod crud_operations {
        use app_error::DomainError;

        use crate::{dao_interface::DAO, database_interface::PostgresPooled};

        pub trait Get<T, K>: DAO {
            type Output;
            fn get(conn: &mut PostgresPooled, id: K) -> Result<Self::Output, DomainError>;
        }

        pub trait GetAll<T>: DAO {
            type Output;
            fn get_all(conn: &mut PostgresPooled) -> Result<Vec<Self::Output>, DomainError>;
        }

        pub trait Insert<T>: DAO {
            type Output;
            fn insert(conn: &mut PostgresPooled, item: T) -> Result<Self::Output, DomainError>;
        }

        pub trait Update<T, Id>: DAO {
            type Output;
            fn update(
                conn: &mut PostgresPooled,
                id: Id,
                item: T,
            ) -> Result<Self::Output, DomainError>;
        }

        #[allow(dead_code)]
        pub trait Delete<T, K>: DAO {
            type Output;
            fn delete(conn: &mut PostgresPooled, item: K) -> Result<Self::Output, DomainError>;
        }
    }
}

#[cfg(feature = "dto")]
pub mod dto_interface {
    pub trait DTO {}
}

#[cfg(feature = "services")]
pub mod service_interface {
    use std::sync::Arc;

    use app_error::AppResult;
    use async_trait::async_trait;
    use tauri::State;
    use tokio::sync::RwLock;

    use crate::{database_interface::DatabaseManagerTrait, dto_interface::DTO};

    pub trait SelectedEdificioTrait {
        fn new() -> Self
        where
            Self: Sized;
        fn set_chiave(&mut self, chiave: String);
        fn get_chiave(&self) -> Option<String>;
        fn clear_chiave(&mut self);
    }

    pub type SelectedEdificioState<T> = Arc<RwLock<T>>;

    #[async_trait]
    pub trait CreateService<T>
    where
        T: DTO,
    {
        async fn create(
            db_state: State<'_, impl DatabaseManagerTrait + Send + Sync>,
            item: T,
        ) -> AppResult<T>;
    }

    #[async_trait]
    pub trait CreateBatchService<T>
    where
        T: DTO,
    {
        async fn create_batch(
            db_state: State<'_, impl DatabaseManagerTrait + Send + Sync>,
            item: Vec<T>,
        ) -> AppResult<Vec<T>>;
    }

    #[async_trait]
    pub trait RetrieveOneService<T, K>
    where
        T: DTO,
    {
        async fn retrieve_one(
            db_state: State<'_, impl DatabaseManagerTrait + Send + Sync>,
            id: K,
        ) -> AppResult<T>;
    }

    #[async_trait]
    pub trait RetrieveBy<T>
    where
        T: DTO,
    {
        type Output;

        async fn retrieve_by(
            db_state: State<'_, impl DatabaseManagerTrait + Send + Sync>,
            where_field: &str,
            where_value: &str,
        ) -> AppResult<Self::Output>;
    }

    #[async_trait]
    pub trait RetrieveByEdificioSelected<T>: RetrieveBy<T>
    where
        T: DTO,
    {
        async fn retrieve_by_edificio_selected<S>(
            db_state: State<'_, impl DatabaseManagerTrait + Send + Sync>,
            edificio_selected_state: State<'_, SelectedEdificioState<S>>,
        ) -> AppResult<Vec<T>>
        where
            S: SelectedEdificioTrait + Send + Sync;
    }

    #[async_trait]
    pub trait RetrieveManyService<T>
    where
        T: DTO,
    {
        async fn retrieve_many(
            db_state: State<'_, impl DatabaseManagerTrait + Send + Sync>,
        ) -> AppResult<Vec<T>>;
    }

    #[async_trait]
    pub trait UpdateService<T>
    where
        T: DTO,
    {
        async fn update(
            db_state: State<'_, impl DatabaseManagerTrait + Send + Sync>,
            item: T,
        ) -> AppResult<T>;
    }

    #[allow(dead_code)]
    #[async_trait]
    pub trait DeleteService<T, K>
    where
        T: DTO,
    {
        async fn delete(
            db_state: State<'_, impl DatabaseManagerTrait + Send + Sync>,
            id: K,
        ) -> AppResult<bool>;
    }
}
