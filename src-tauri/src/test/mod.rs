#[cfg(test)]
mod tests {
    use crate::app_traits::{
        CreateTable, DaoTrait, EntityTrait, FromRow, GetAll, GetById, Insert, SqlParams, ToInsert,
        ToRetrieve, ToRetrieveAll, ToUpdate, Update,
    };
    use crate::db::Database;
    use crate::utils::AppError;
    use rusqlite::Row;

    #[derive(Debug, PartialEq, Clone)]
    struct TestEntity {
        name: String,
        age: i32,
    }

    impl FromRow for TestEntity {
        fn from_row(row: &Row) -> Result<Self, rusqlite::Error> {
            Ok(Self {
                name: row.get("NAME")?,
                age: row.get("AGE")?,
            })
        }
    }

    impl EntityTrait for TestEntity {
        type PrimaryKey = String;

        fn table_name() -> String {
            "test_table".to_string()
        }

        fn sql_create_table() -> String {
            format!(
                "CREATE TABLE {} (NAME TEXT PRIMARY KEY, AGE INTEGER) STRICT;",
                Self::table_name()
            )
        }
    }

    impl ToInsert for TestEntity {
        fn to_insert() -> String {
            format!(
                "INSERT INTO {} (NAME, AGE) VALUES (?, ?) RETURNING *;",
                Self::table_name()
            )
        }

        fn to_insert_params(&self) -> Vec<&dyn SqlParams> {
            vec![&self.name, &self.age]
        }
    }

    impl ToRetrieveAll for TestEntity {}

    impl ToRetrieve for TestEntity {
        fn to_retrieve() -> String {
            format!("SELECT * FROM {} WHERE NAME = ?;", Self::table_name())
        }
    }

    impl ToUpdate for TestEntity {
        fn to_update() -> String {
            format!(
                "UPDATE {} SET AGE = ? WHERE NAME = ? RETURNING *",
                Self::table_name()
            )
        }

        fn to_update_params(&self) -> Vec<Box<&dyn SqlParams>> {
            vec![Box::new(&self.age), Box::new(&self.name)]
        }
    }

    struct TestDao;

    impl DaoTrait for TestDao {
        type Entity = TestEntity;
        type Error = AppError;
    }

    impl CreateTable for TestDao {}

    impl GetAll for TestDao {}
    impl GetById for TestDao {}
    impl Insert for TestDao {}
    impl Update for TestDao {}

    #[test]
    fn test_create_table() {
        let database = Database::open_in_memory();
        let conn_guard = database.get_conn().unwrap();
        let conn = conn_guard.as_ref().unwrap();

        match TestDao::create_table(conn) {
            Ok(_) => (),
            Err(err) => panic!("Error creating table: {}", err),
        }

        pretty_sqlite::print_table(conn, &TestDao::table_name()).unwrap()
    }

    #[test]
    fn test_insert() {
        let database = Database::open_in_memory();
        let conn_guard = database.get_conn().unwrap();
        let conn = conn_guard.as_ref().unwrap();

        let entity = TestEntity {
            name: "test_name".to_string(),
            age: 10,
        };

        TestDao::create_table(conn).unwrap();

        match TestDao::insert(conn, entity.clone()) {
            Ok(res_entity) => {
                assert_eq!(entity, res_entity)
            }
            Err(err) => panic!("Error inserting entity: {}", err),
        }

        pretty_sqlite::print_table(conn, &TestDao::table_name()).unwrap()
    }

    #[test]
    fn test_multi_insert() {
        let database = Database::open_in_memory();
        let conn_guard = database.get_conn().unwrap();
        let conn = conn_guard.as_ref().unwrap();

        let mut entities = Vec::with_capacity(10);
        for i in 0..10 {
            entities.push(TestEntity {
                name: format!("test_name{}", i),
                age: i,
            });
        }

        TestDao::create_table(conn).unwrap();

        for entity in entities.iter() {
            match TestDao::insert(conn, entity.clone()) {
                Ok(res_entity) => {
                    assert_eq!(*entity, res_entity)
                }
                Err(err) => panic!("Error inserting entity: {}", err),
            }
        }

        pretty_sqlite::print_table(conn, &TestDao::table_name()).unwrap()
    }

    #[test]
    fn test_insert_with_error_primary_key_exists() {
        let database = Database::open_in_memory();
        let conn_guard = database.get_conn().unwrap();
        let conn = conn_guard.as_ref().unwrap();

        let entity = TestEntity {
            name: "test_name".to_string(),
            age: 10,
        };
        TestDao::create_table(conn).unwrap();
        TestDao::insert(conn, entity.clone()).unwrap();
        match TestDao::insert(conn, entity.clone()) {
            Err(AppError::DatabaseError(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error { code, .. },
                ..,
            ))) => {
                assert_eq!(code, rusqlite::ffi::ErrorCode::ConstraintViolation);
            }
            _ => panic!("Error inserting entity"),
        }
    }

    #[test]
    fn test_get_all() {
        let database = Database::open_in_memory();
        let conn_guard = database.get_conn().unwrap();
        let conn = conn_guard.as_ref().unwrap();

        let mut entities = Vec::with_capacity(10);
        for i in 0..10 {
            entities.push(TestEntity {
                name: format!("test_name{}", i),
                age: i,
            });
        }

        TestDao::create_table(conn).unwrap();

        for entity in entities.iter() {
            TestDao::insert(conn, entity.clone()).unwrap();
        }

        match TestDao::get_all(conn) {
            Ok(entities_res) => {
                assert_eq!(entities, entities_res);
            }
            Err(err) => panic!("Error getting all entities: {}", err),
        }

        pretty_sqlite::print_table(conn, &TestDao::table_name()).unwrap()
    }

    #[test]
    fn test_get_by_id() {
        let database = Database::open_in_memory();
        let conn_guard = database.get_conn().unwrap();
        let conn = conn_guard.as_ref().unwrap();

        let mut entities = Vec::with_capacity(10);
        for i in 0..10 {
            entities.push(TestEntity {
                name: format!("test_name{}", i),
                age: i,
            });
        }

        TestDao::create_table(conn).unwrap();

        for entity in entities.iter() {
            TestDao::insert(conn, entity.clone()).unwrap();
        }

        match TestDao::get_by_id(conn, "test_name5".to_string()) {
            Ok(entities_res) => {
                println!("{:?}", entities_res);
                assert_eq!(*entities.get(5).unwrap(), entities_res);
            }
            Err(err) => panic!("Error getting all entities: {}", err),
        }
    }

    #[test]
    fn test_update() {
        let database = Database::open_in_memory();
        let conn_guard = database.get_conn().unwrap();
        let conn = conn_guard.as_ref().unwrap();
        let entity = TestEntity {
            name: "test_name".to_string(),
            age: 10,
        };

        TestDao::create_table(conn).unwrap();
        TestDao::insert(conn, entity.clone()).unwrap();

        pretty_sqlite::print_table(conn, &TestDao::table_name()).unwrap();

        let entity = TestEntity {
            name: "test_name".to_string(),
            age: 25,
        };

        match TestDao::update(conn, entity.clone()) {
            Ok(res_entity) => {
                assert_eq!(entity, res_entity)
            }
            Err(err) => panic!("Error updating entity: {}", err),
        }

        pretty_sqlite::print_table(conn, &TestDao::table_name()).unwrap()
    }
}
