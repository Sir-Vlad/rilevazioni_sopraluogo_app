//mod database_connection;
pub mod database_manager;
mod error;
//mod migrations;
mod utils;

//pub use database_connection::DatabaseConnection;
pub use database_manager::DatabaseConnector;
pub use database_manager::DatabaseManager;
pub use error::{DataMigrationError, DbError, MigrationError};
//pub use utils::{convert_param, get_db_path, init_database, set_pragma, NAME_DIR_DATABASE};
//pub use migrations::*;