use crate::app_traits::{
    entity::{
        EntityTrait, FromRow, ToCrud, ToDelete, ToInsert, ToRetrieve, ToRetrieveAll, ToUpdate,
    },
    sql_executor::SqlExecutor,
    sql_params::SqlParams,
};
use rusqlite::params;

pub trait DaoTrait {
    type Entity: EntityTrait;
    type Error: From<rusqlite::Error>;

    fn table_name() -> String {
        Self::Entity::table_name()
    }
}

pub trait CreateTable: DaoTrait {
    fn create_table<Connection: SqlExecutor>(conn: &Connection) -> Result<(), Self::Error> {
        conn.execute(&Self::Entity::sql_create_table(), &[])?;
        Ok(())
    }
}

pub trait CreateView: DaoTrait {
    fn create_table<Connection: SqlExecutor>(conn: &Connection) -> Result<(), Self::Error> {
        conn.execute(&Self::Entity::sql_create_table(), &[])?;
        Ok(())
    }
}

pub trait GetAll: DaoTrait
where
    Self::Entity: ToRetrieveAll,
{
    /// Retrieves all records of an entity from the db.
    ///
    /// This function interacts with the db connection to retrieve all records
    /// of the specified entity type. It prepares a SQL query, executes it, and
    /// maps the resulting rows to their corresponding entity type. If successful,
    /// it returns a `Vec` of entities. If an error occurs during the process, the
    /// error is propagated.
    ///
    /// # Type Parameters
    /// - `Self`: The struct implementing this method, typically representing a data access
    ///   layer for a specific entity type.
    /// - `Self::Connection`: The type of the db connection.
    /// - `Self::Entity`: The type of entity being retrieved.
    /// - `Self::Error`: The error type that this function returns in case of failure.
    ///
    /// # Arguments
    /// - `conn`: An instance of the db connection that will be used to run the query.
    ///
    /// # Returns
    /// - `Ok(Vec<Self::Entity>)`: A vector of entities if the query and mapping operations succeed.
    /// - `Err(Self::Error)`: An error if the query preparation, execution, or row mapping fails.
    ///
    /// # Errors
    /// This function will return an error in any of the following scenarios:
    /// - The SQL query preparation fails.
    /// - The execution of the SQL query fails.
    /// - The mapping of rows to entity objects encounters errors.
    ///
    /// # Example
    /// Assuming `MyEntity` implements the required traits (e.g., `to_retrieve_all`, `from_row`),
    /// and `MyConnection` is the associated db connection:
    ///
    /// ```rust
    /// let connection = MyConnection::new();
    /// match MyRepository::get_all(connection) {
    ///     Ok(entities) => {
    ///         for entity in entities {
    ///             println!("{:?}", entity);
    ///         }
    ///     }
    ///     Err(e) => {
    ///         eprintln!("Error retrieving entities: {}", e);
    ///     }
    /// }
    /// ```
    ///
    /// # Requirements
    /// - The `Self::Entity` struct must implement a method named `to_retrieve_all`, which
    ///   returns the SQL query string to fetch all records.
    /// - The `Self::Entity` struct must also implement a method named `from_row`,
    ///   which maps a db row to an entity instance.
    /// - The db connection should be compatible with the query execution and
    ///   parameter binding used in this function.
    fn get_all<Connection: SqlExecutor>(
        conn: &Connection,
    ) -> Result<Vec<Self::Entity>, Self::Error> {
        let mut stmt = conn.prepare(Self::Entity::to_retrieve_all().as_str())?;
        let result: Result<Vec<Self::Entity>, _> =
            stmt.query_map(params![], Self::Entity::from_row)?.collect();
        match result {
            Ok(result) => Ok(result),
            Err(err) => Err(err.into()),
        }
    }
}

pub trait GetById: DaoTrait
where
    Self::Entity: ToRetrieve,
{
    /// Retrieves a single entity by its primary key from the db.
    ///
    /// This function is a wrapper designed to fetch a single record from the db
    /// based on the primary key of the target entity. If the entity is found, it is returned
    /// as a result. If no record matches the provided primary key, an error is returned.
    ///
    /// # Parameters
    ///
    /// - `conn`: The db connection used to perform the query. Must implement the
    ///   `Self::Connection` type.
    /// - `id`: The primary key of the entity to retrieve. The type of the primary key
    ///   is defined by the `EntityTrait` implementation for `Self::Entity`.
    ///
    /// # Returns
    ///
    /// - `Ok(Self::Entity)`: If a matching record is found, the entity is returned as the successful `Result`.
    /// - `Err(Self::Error)`: If no record is found, if there is an issue executing the query,
    ///   or if there is an issue mapping the rows to the entity, an appropriate error is returned.
    ///
    /// # Errors
    ///
    /// - `Self::Error::from(rusqlite::Error::QueryReturnedNoRows)`: If no rows are returned from the query.
    /// - `Self::Error`: Based on any other errors encountered during the preparation, execution
    ///   of the query, or during the mapping of rows to an entity.
    ///
    /// # Implementation Details
    ///
    /// - The SQL query is generated using `Self::Entity::to_retrieve()` which defines
    ///   the appropriate SELECT statement for the entity.
    /// - The primary key value is converted to SQL parameters using `id.to_sql_params()`.
    /// - The query result is mapped into the entity using `Self::Entity::from_row()` for every row
    ///   in the query result.
    /// - If the query returns more than one row, the function still returns only the first entity found.
    ///
    /// # Example
    ///
    /// ```rust
    /// let conn = DatabaseConnection::new();
    /// let id = 1; // Primary key of the entity to retrieve
    /// match MyEntity::get_by_id(conn, id) {
    ///     Ok(entity) => println!("Entity retrieved: {:?}", entity),
    ///     Err(error) => eprintln!("Error retrieving entity: {:?}", error),
    /// }
    /// ```
    fn get_by_id<Connection: SqlExecutor>(
        conn: &Connection,
        id: <Self::Entity as EntityTrait>::PrimaryKey,
    ) -> Result<Self::Entity, Self::Error> {
        let sql = Self::Entity::to_retrieve();
        let param = id.to_sql_params();

        let mut stmt = conn.prepare(sql.as_str())?;
        let result: Result<Vec<Self::Entity>, _> = stmt
            .query_map(rusqlite::params_from_iter(param), |row| {
                Self::Entity::from_row(row)
            })?
            .collect();

        match result {
            Ok(result) => {
                if result.is_empty() {
                    Err(Self::Error::from(rusqlite::Error::QueryReturnedNoRows))
                } else {
                    Ok(result.into_iter().next().unwrap())
                }
            }
            Err(e) => Err(Self::Error::from(e)),
        }
    }
}

pub trait Insert: DaoTrait
where
    Self::Entity: ToInsert,
{
    ///
    /// Inserts a new entity into the db.
    ///
    /// This function takes a db connection and an entity instance, then performs an
    /// insertion into the corresponding db table. The insertion query is generated
    /// from the `Entity` trait's `to_insert` method, and the parameters needed for the
    /// query are resolved using the entity's `to_insert_params` method.
    ///
    /// After inserting, the function retrieves the newly created entity from the db
    /// and converts it back to the entity type using the `from_row` method of the `Entity` trait.
    ///
    /// # Parameters
    /// - `conn`: The established db connection (`Self::Connection`) used to interact with the db.
    /// - `entity`: The entity (`Self::Entity`) to be inserted into the db.
    ///
    /// # Returns
    /// A `Result` containing the newly inserted entity (`Self::Entity`) on success, or an error of type
    /// `Self::Error` in case of any failure during the preparation, insertion, or retrieval process.
    ///
    /// # Errors
    /// This function may return an error in the following cases:
    /// - The insertion query fails to prepare (`conn.prepare` returns an error).
    /// - Parameter binding fails (`to_sql_params()` or related operations cause issues).
    /// - The query fails to execute (`stmt.query_row` fails).
    ///
    /// # Example
    /// ```rust
    /// let conn = establish_connection(); // Assumes a function to establish the db connection.
    /// let entity = MyEntity::new(...);  // Create an instance of the entity to insert.
    ///
    /// match MyDatabase::insert(conn, entity) {
    ///     Ok(new_entity) => println!("Inserted entity: {:?}", new_entity),
    ///     Err(err) => eprintln!("Error inserting entity: {:?}", err),
    /// }
    /// ```
    ///
    /// # Notes
    /// - The `Entity` trait must be implemented for the type of the entity you want to insert.
    /// - The connection and the entity type must both statically conform to the expected `Self::Connection`
    ///   and `Self::Entity` types defined for the structure or module this function is implemented within.
    /// - The query must contain the keyword `RETURNING *`
    fn insert<Connection: SqlExecutor>(
        conn: &Connection,
        entity: Self::Entity,
    ) -> Result<Self::Entity, Self::Error> {
        let query = Self::Entity::to_insert();
        if !query.contains("RETURNING *") {
            panic!("Insert query must contain RETURNING *");
        }

        let params = entity.to_insert_params();
        let sql_params = params
            .into_iter()
            .flat_map(|p| p.to_sql_params())
            .collect::<Vec<_>>();

        let mut stmt = conn.prepare(&query)?;
        let new_entity = stmt.query_row(rusqlite::params_from_iter(sql_params), |row| {
            Self::Entity::from_row(row)
        })?;

        Ok(new_entity)
    }
}

pub trait Update: DaoTrait
where
    Self::Entity: ToUpdate,
{
    /// Updates an existing entity in the db using the provided connection.
    ///
    /// This function performs the following steps:
    /// 1. Converts the entity into a query string and update parameters.
    /// 2. Prepares the SQL query using the provided connection.
    /// 3. Executes the query using the parameters and extracts the updated entity from the result row.
    ///
    /// # Type Parameters
    /// - `Connection`: A type that implements the `SqlExecutor` trait, used for db interaction.
    ///
    /// # Parameters
    /// - `conn`: A reference to the db connection implementing the `SqlExecutor` trait.
    /// - `entity`: The entity to be updated in the db. The provided entity specifies the values for the update query.
    ///
    /// # Returns
    /// - `Ok(Self::Entity)`: The updated entity retrieved from the db after successful execution of the query.
    /// - `Err(Self::Error)`: An error if the query execution or row extraction fails.
    ///
    /// # Errors
    /// Returns an error in the following scenarios:
    /// - If there is an issue preparing the SQL statement.
    /// - If there is an issue executing the query, such as constraint violations or query syntax errors.
    /// - If there is an issue mapping the result row back into the entity.
    ///
    /// # Notes
    /// - `Self::Entity` must implement the `to_update` and `from_row` methods:
    ///   - `to_update` is expected to return the SQL update statement.
    ///   - `from_row` is expected to deserialize the db row back into the entity type.
    /// - The `to_update_params` method is expected to convert the entity into a collection
    ///   of parameters for the SQL update query.
    fn update<Connection: SqlExecutor>(
        conn: &Connection,
        entity: Self::Entity,
    ) -> Result<Self::Entity, Self::Error> {
        let (query, params) = if let Some((query, params)) = entity.to_build_update() {
            (query, params)
        } else {
            (Self::Entity::to_update(), entity.to_update_params())
        };
        if !query.contains("RETURNING *") {
            panic!("Insert query must contain RETURNING *");
        }

        let sql_params = params
            .into_iter()
            .flat_map(|p| p.to_sql_params())
            .collect::<Vec<_>>();

        let mut stmt = conn.prepare(&query)?;
        let new_entity = stmt.query_row(rusqlite::params_from_iter(sql_params), |row| {
            Self::Entity::from_row(row)
        })?;

        Ok(new_entity)
    }
}

pub trait Delete: DaoTrait
where
    Self::Entity: ToDelete,
{
    fn delete<Connection: SqlExecutor>(
        conn: &Connection,
        id: <Self::Entity as EntityTrait>::PrimaryKey,
    ) -> Result<bool, Self::Error>;
}

pub trait ReadOnlyDao: GetAll + GetById
where
    Self::Entity: ToRetrieveAll + ToRetrieve,
{
}

impl<T> ReadOnlyDao for T
where
    T: GetAll + GetById,
    Self::Entity: ToRetrieveAll + ToRetrieve,
{
}

pub trait WriteDao: Insert + Update
where
    Self::Entity: ToInsert + ToUpdate,
{
}
impl<T> WriteDao for T
where
    T: Insert + Update,
    Self::Entity: ToInsert + ToUpdate,
{
}

pub trait CrudDao: ReadOnlyDao + WriteDao + Delete
where
    Self::Entity: ToCrud,
{
}
impl<T> CrudDao for T
where
    T: ReadOnlyDao + WriteDao + Delete,
    Self::Entity: ToCrud,
{
}
