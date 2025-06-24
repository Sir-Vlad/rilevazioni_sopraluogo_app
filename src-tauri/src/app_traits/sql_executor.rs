use rusqlite::ToSql;

pub trait SqlExecutor {
    fn execute(&self, sql: &str, params: &[&dyn ToSql]) -> rusqlite::Result<usize>;
    fn prepare(&self, sql: &str) -> rusqlite::Result<rusqlite::Statement>;
}
