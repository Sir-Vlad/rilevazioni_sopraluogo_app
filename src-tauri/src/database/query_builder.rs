pub enum QueryType {
    Select,
    Insert,
    Update,
    Delete,
}
pub struct QueryBuilder {
    query_type: QueryType,
    table: Option<String>,
    columns: Vec<String>,
    values: Vec<Vec<QueryParam>>,
    selected_columns: Option<String>,
    where_clauses: Vec<WhereClause>,
    order_by: Option<String>,
    limit: Option<usize>,
    offset: Option<usize>,
    returning: Option<String>,
    set_clauses: Vec<(String, QueryParam)>,
}

struct WhereClause {
    condition: String,
    params: Vec<QueryParam>,
}

impl Default for QueryBuilder {
    fn default() -> Self {
        QueryBuilder {
            query_type: QueryType::Select,
            table: None,
            columns: Vec::new(),
            values: Vec::new(),
            selected_columns: None,
            where_clauses: Vec::new(),
            order_by: None,
            limit: None,
            offset: None,
            returning: None,
            set_clauses: Vec::new(),
        }
    }
}

impl QueryBuilder {
    pub fn select() -> Self {
        Self {
            query_type: QueryType::Select,
            ..Default::default()
        }
    }

    pub fn update() -> Self {
        Self {
            query_type: QueryType::Update,
            ..Default::default()
        }
    }

    pub fn insert() -> Self {
        Self {
            query_type: QueryType::Insert,
            ..Default::default()
        }
    }

    pub fn delete() -> Self {
        Self {
            query_type: QueryType::Delete,
            ..Default::default()
        }
    }

    pub fn query(type_query: QueryType) -> Self {
        match type_query {
            QueryType::Select => Self::select(),
            QueryType::Insert => Self::insert(),
            QueryType::Update => Self::update(),
            QueryType::Delete => Self::delete(),
        }
    }

    // methods commons to all query types
    pub fn table(mut self, table: &str) -> Self {
        self.table = Some(table.to_string());
        self
    }

    pub fn returning(mut self, columns: &str) -> Self {
        self.returning = Some(columns.to_string());
        self
    }

    // methods specific to select query type
    pub fn columns(mut self, columns: &str) -> Self {
        self.selected_columns = Some(columns.to_string());
        self
    }

    pub fn order_by(mut self, order_by: &str) -> Self {
        self.order_by = Some(order_by.to_string());
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    // methods specific to insert query type
    pub fn values(mut self, values: Vec<QueryParam>) -> Self {
        self.values.push(values);
        self
    }

    pub fn into_columns(mut self, columns: Vec<&str>) -> Self {
        self.columns = columns.into_iter().map(|c| c.to_string()).collect();
        self
    }

    // methods specific to update query type
    pub fn set<T: Into<QueryParam>>(mut self, column: &str, value: T) -> Self {
        self.set_clauses.push((column.to_string(), value.into()));
        self
    }

    pub fn set_if<T: Into<QueryParam>>(mut self, column: &str, value: Option<T>) -> Self {
        if let Some(val) = value {
            self.set_clauses.push((column.to_string(), val.into()));
        }
        self
    }

    // methods common to where clause
    pub fn where_clause(mut self, condition: &str) -> Self {
        self.where_clauses.push(WhereClause {
            condition: condition.to_string(),
            params: Vec::new(),
        });
        self
    }

    pub fn where_eq<T: Into<QueryParam>>(mut self, column: &str, value: T) -> Self {
        let param = value.into();
        self.where_clauses.push(WhereClause {
            condition: format!("{} = ?", column),
            params: vec![param],
        });
        self
    }

    pub fn where_if<T: Into<QueryParam>>(mut self, column: &str, value: Option<T>) -> Self {
        if let Some(val) = value {
            let param = val.into();
            self.where_clauses.push(WhereClause {
                condition: format!("{} = ?", column),
                params: vec![param],
            });
        }
        self
    }

    pub fn where_in<T: Into<QueryParam>>(mut self, column: &str, values: Vec<T>) -> Self {
        if values.is_empty() {
            return self;
        }

        let mut params = Vec::new();
        let placeholders = (0..values.len())
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(", ");
        for value in values {
            params.push(value.into());
        }

        self.where_clauses.push(WhereClause {
            condition: format!("{} IN ({})", column, placeholders),
            params,
        });
        self
    }
    pub fn build(&self) -> Result<(String, Vec<&QueryParam>), QueryBuilderError> {
        if self.table.is_none() {
            return Err(QueryBuilderError::MissingTable);
        }
        let table = self.table.as_ref().unwrap();
        let mut query = String::new();
        let mut params_refs = Vec::new();

        match &self.query_type {
            QueryType::Select => self.build_select(&mut query, &mut params_refs, table)?,
            QueryType::Insert => self.build_insert(&mut query, &mut params_refs, table)?,
            QueryType::Update => self.build_update(&mut query, &mut params_refs, table)?,
            QueryType::Delete => self.build_delete(&mut query, &mut params_refs, table)?,
        }

        Ok((query.trim().to_string(), params_refs))
    }

    fn build_select<'a>(
        &'a self,
        query: &mut String,
        param_refs: &mut Vec<&'a QueryParam>,
        table: &String,
    ) -> Result<(), QueryBuilderError> {
        query.push_str("SELECT ");
        if let Some(columns) = &self.selected_columns {
            query.push_str(columns);
        } else {
            query.push('*');
        }

        query.push_str(&format!(" FROM {} ", table));

        self.append_common_clauses(query, param_refs)?;

        Ok(())
    }

    fn build_insert<'a>(
        &'a self,
        query: &mut String,
        param_refs: &mut Vec<&'a QueryParam>,
        table: &String,
    ) -> Result<(), QueryBuilderError> {
        if self.columns.is_empty() {
            return Err(QueryBuilderError::MissingColumns);
        }
        if self.values.is_empty() {
            return Err(QueryBuilderError::MissingValues);
        }

        query.push_str(&format!("INSERT INTO {} ", table));
        query.push_str(&format!("({}) ", self.columns.join(", ")));

        query.push_str("VALUES ");
        let mut value_groups = Vec::new();
        let mut param_index = 1;
        for value_row in &self.values {
            if value_row.len() != self.columns.len() {
                return Err(QueryBuilderError::ColumnValueMismatch);
            }

            let mut placeholders = Vec::new();
            for value in value_row {
                placeholders.push(format!("${}", param_index));
                param_refs.push(value);
                param_index += 1;
            }
            value_groups.push(format!("({})", placeholders.join(", ")));
        }
        query.push_str(&value_groups.join(", "));

        if let Some(returning) = &self.returning {
            query.push_str(&format!("RETURNING {}", returning));
        }

        Ok(())
    }

    fn build_update<'a>(
        &'a self,
        query: &mut String,
        params_refs: &mut Vec<&'a QueryParam>,
        table: &String,
    ) -> Result<(), QueryBuilderError> {
        if self.set_clauses.is_empty() {
            return Err(QueryBuilderError::MissingSetClauses);
        }

        // UPDATE
        query.push_str(&format!("UPDATE {} SET ", table));

        // SET clauses
        let mut set_parts = Vec::new();
        let mut param_index = 1;

        for (column, value) in &self.set_clauses {
            set_parts.push(format!("{} = ${}", column, param_index));
            params_refs.push(value);
            param_index += 1;
        }

        query.push_str(&set_parts.join(", "));

        // WHERE
        if !self.where_clauses.is_empty() {
            query.push_str(" WHERE ");

            let where_parts: Vec<String> = Vec::new();
            for where_clause in &self.where_clauses {
                let mut clause_text = where_clause.condition.clone();
                while let Some(pos) = clause_text.find("?") {
                    clause_text = clause_text[..pos].to_string()
                        + &format!("${}", param_index)
                        + &clause_text[pos + 1..];
                    param_index += 1;
                }

                query.push_str(&clause_text);

                for param in &where_clause.params {
                    params_refs.push(param);
                }
            }

            query.push_str(&where_parts.join(" AND "));
        }

        // RETURNING
        if let Some(returning) = &self.returning {
            query.push_str(&format!(" RETURNING {}", returning));
        }
        Ok(())
    }

    fn build_delete<'a>(
        &'a self,
        query: &mut String,
        params_refs: &mut Vec<&'a QueryParam>,
        table: &String,
    ) -> Result<(), QueryBuilderError> {
        // DELETE FROM
        query.push_str(&format!("DELETE FROM {} ", table));

        // WHERE
        let mut param_index = 1;
        self.build_where_clause(query, params_refs, &mut param_index);

        // RETURNING
        if let Some(returning) = &self.returning {
            query.push_str(&format!(" RETURNING {}", returning));
        }

        Ok(())
    }

    fn build_where_clause<'a>(
        &'a self,
        query: &mut String,
        params_refs: &mut Vec<&'a QueryParam>,
        param_index: &mut i32,
    ) {
        if !self.where_clauses.is_empty() {
            query.push_str("WHERE ");

            let mut where_parts = Vec::new();

            for where_clause in &self.where_clauses {
                // Sostituisci ? con $N dove N è l'indice corrente del parametro
                let mut clause_text = where_clause.condition.clone();
                while let Some(pos) = clause_text.find('?') {
                    clause_text = clause_text[..pos].to_string()
                        + &format!("${}", param_index)
                        + &clause_text[pos + 1..];
                    *param_index += 1;
                }

                where_parts.push(clause_text);

                // Aggiungi i parametri associati a questa clausola WHERE
                for param in &where_clause.params {
                    params_refs.push(param);
                }
            }

            query.push_str(&where_parts.join(" AND "));
        }
    }

    fn append_common_clauses<'a>(
        &'a self,
        query: &mut String,
        params_refs: &mut Vec<&'a QueryParam>,
    ) -> Result<(), QueryBuilderError> {
        // WHERE
        let mut param_index = 1;
        self.build_where_clause(query, params_refs, &mut param_index);

        if let Some(order_by) = &self.order_by {
            query.push_str(&format!("ORDER BY {} ", order_by));
        }

        if let Some(limit) = &self.limit {
            query.push_str(&format!("LIMIT {} ", limit));
        }

        if let Some(offset) = &self.offset {
            query.push_str(&format!("ORDER BY {} ", offset));
        }

        Ok(())
    }

    pub fn from_dto<T: FilterDTO>(dto: &T, table: &str) -> Self {
        let mut builder = Self::select().table(table);
        dto.apply_filter(&mut builder);
        builder
    }
}

pub trait FilterDTO {
    fn apply_filter(&self, builder: &mut QueryBuilder);
}

pub enum QueryParam {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
}
impl From<String> for QueryParam {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
impl From<&str> for QueryParam {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}
impl From<i64> for QueryParam {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
impl From<i32> for QueryParam {
    fn from(value: i32) -> Self {
        Self::Integer(value as i64)
    }
}
impl From<i16> for QueryParam {
    fn from(value: i16) -> Self {
        Self::Integer(value as i64)
    }
}
impl From<i8> for QueryParam {
    fn from(value: i8) -> Self {
        Self::Integer(value as i64)
    }
}
impl From<u64> for QueryParam {
    fn from(value: u64) -> Self {
        Self::Integer(value as i64)
    }
}
impl From<u32> for QueryParam {
    fn from(value: u32) -> Self {
        Self::Integer(value as i64)
    }
}
impl From<u16> for QueryParam {
    fn from(value: u16) -> Self {
        Self::Integer(value as i64)
    }
}
impl From<u8> for QueryParam {
    fn from(value: u8) -> Self {
        Self::Integer(value as i64)
    }
}
impl From<f64> for QueryParam {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}
impl From<bool> for QueryParam {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl Clone for QueryParam {
    fn clone(&self) -> Self {
        match self {
            QueryParam::String(s) => QueryParam::String(s.clone()),
            QueryParam::Integer(i) => QueryParam::Integer(*i),
            QueryParam::Float(f) => QueryParam::Float(*f),
            QueryParam::Boolean(b) => QueryParam::Boolean(*b),
            QueryParam::Null => QueryParam::Null,
        }
    }
}
#[derive(Debug)]
pub enum QueryBuilderError {
    MissingFrom,
    MissingTable,
    MissingColumns,
    MissingValues,
    ColumnValueMismatch,
    MissingSetClauses,
}

impl std::fmt::Display for QueryBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingFrom => write!(f, "Missing FROM clause"),
            Self::MissingTable => write!(f, "Missing table name"),
            Self::MissingColumns => write!(f, "Missing columns"),
            Self::MissingValues => write!(f, "Missing values"),
            Self::ColumnValueMismatch => write!(f, "Column value mismatch"),
            Self::MissingSetClauses => write!(f, "Missing SET clauses"),
        }
    }
}

impl std::error::Error for QueryBuilderError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_query() {
        let binding = QueryBuilder::select().table("users").columns("id, name");
        let query = binding.build();
        assert_eq!(query.unwrap().0, "SELECT id, name FROM users");
    }

    #[test]
    fn test_insert_query() {
        let binding = QueryBuilder::insert()
            .table("users")
            .into_columns(vec!["name", "age"])
            .values(vec!["John".into(), 30.into()]);
        let query = binding.build();
        assert_eq!(
            query.unwrap().0,
            "INSERT INTO users (name, age) VALUES ($1, $2)"
        );
    }

    #[test]
    fn test_update_query() {
        let binding = QueryBuilder::update()
            .table("users")
            .set("name", "John")
            .set("age", 30)
            .where_eq("id", 1);
        let query = binding.build();
        assert_eq!(
            query.unwrap().0,
            "UPDATE users SET name = $1, age = $2 WHERE id = $3"
        );
    }

    #[test]
    fn test_delete_query() {
        let binding = QueryBuilder::delete().table("users").where_eq("id", 1);
        let query = binding.build();
        assert_eq!(query.unwrap().0, "DELETE FROM users WHERE id = $1");
    }

    #[test]
    fn test_where_clause() {
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
    fn test_query_errors() {
        assert!(matches!(
            QueryBuilder::select().build(),
            Err(QueryBuilderError::MissingTable)
        ));

        assert!(matches!(
            QueryBuilder::insert().table("users").build(),
            Err(QueryBuilderError::MissingColumns)
        ));

        assert!(matches!(
            QueryBuilder::update().table("users").build(),
            Err(QueryBuilderError::MissingSetClauses)
        ));
    }

    #[test]
    fn test_build_delete_con_multiple_placeholders() {
        // Test con una clausola WHERE che contiene più ? nella stessa condizione
        let mut builder = QueryBuilder::delete().table("prodotti");
        builder.where_clauses.push(WhereClause {
            condition: "nome LIKE ? OR descrizione LIKE ? OR categoria = ?".to_string(),
            params: vec![
                QueryParam::String("%test%".into()),
                QueryParam::String("%search%".into()),
                QueryParam::String("elettronica".into()),
            ],
        });

        // Esegui il build della query
        let result = builder.build().unwrap();
        let (query, params) = result;

        // Verifica la query e i parametri
        assert_eq!(
            query,
            "DELETE FROM prodotti WHERE nome LIKE $1 OR descrizione LIKE $2 OR categoria = $3"
        );
        assert_eq!(params.len(), 3);

        if let QueryParam::String(val) = params[0] {
            assert_eq!(val, "%test%");
        } else {
            panic!("Tipo di parametro errato per il primo parametro");
        }

        if let QueryParam::String(val) = params[1] {
            assert_eq!(val, "%search%");
        } else {
            panic!("Tipo di parametro errato per il secondo parametro");
        }

        if let QueryParam::String(val) = params[2] {
            assert_eq!(val, "elettronica");
        } else {
            panic!("Tipo di parametro errato per il terzo parametro");
        }
    }
}
