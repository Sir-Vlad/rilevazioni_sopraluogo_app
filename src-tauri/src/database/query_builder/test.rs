#[cfg(test)]
mod tests {
    use crate::database::query_builder::where_builder::WhereBuilder;
    use crate::database::query_builder::SqlQueryBuilder;
    use crate::database::QueryBuilder;

    #[test]
    fn test_select_query() {
        let binding = QueryBuilder::select()
            .table("users")
            .columns(vec!["id", "name"]);
        let query = binding.build();
        assert_eq!(query.unwrap().0, "SELECT id, name FROM users");
    }

    #[test]
    fn test_select_with_where() {
        let binding = QueryBuilder::select()
            .table("users")
            .where_eq("name", "John")
            .where_eq("age", 30);
        let query = binding.build();
        assert_eq!(
            query.unwrap().0,
            "SELECT * FROM users WHERE name = $1 AND age = $2"
        );
    }

    #[test]
    fn test_select_with_or_condition() {
        let binding = QueryBuilder::select()
            .table("users")
            .where_eq("name", "John")
            .where_eq("name", "Jane")
            .with_or();
        let query = binding.build();
        assert_eq!(
            query.unwrap().0,
            "SELECT * FROM users WHERE name = $1 OR name = $2"
        );
    }

    // Altri test...
}
