use crate::{
    database::query_builder::SqlQueryBuilder,
    database::{QueryBuilderError, QueryParam},
};

#[derive(Debug)]
pub struct InsertQueryBuilder {
    table: Option<String>,
    columns: Vec<String>,
    values: Vec<Vec<QueryParam>>,
    returning: Option<String>,
}

impl InsertQueryBuilder {
    pub fn new() -> Self {
        InsertQueryBuilder {
            table: None,
            columns: Vec::new(),
            values: Vec::new(),
            returning: None,
        }
    }

    pub fn table(mut self, table: &str) -> Self {
        self.table = Some(table.to_string());
        self
    }

    pub fn columns(mut self, columns: Vec<&str>) -> Self {
        self.columns = columns.into_iter().map(|c| c.to_string()).collect();
        self
    }

    pub fn values(mut self, values: Vec<QueryParam>) -> Self {
        self.values.push(values);
        self
    }

    pub fn batch_values(mut self, batch_values: Vec<Vec<QueryParam>>) -> Self {
        self.values.extend(batch_values);
        self
    }

    #[allow(unused_must_use, dead_code, unused_variables, unused_mut)]
    pub fn set(mut self, column: &str, values: Vec<QueryParam>) -> Self {
        todo!()
    }

    pub fn returning(mut self, returning: &str) -> Self {
        self.returning = Some(returning.to_string());
        self
    }
}

impl SqlQueryBuilder for InsertQueryBuilder {
    fn build(&self) -> Result<(String, Vec<&QueryParam>), QueryBuilderError> {
        if self.table.is_none() {
            return Err(QueryBuilderError::MissingTable);
        }

        if self.columns.is_empty() {
            return Err(QueryBuilderError::MissingColumns);
        }

        if self.values.is_empty() {
            return Err(QueryBuilderError::MissingValues);
        }

        let mut query = String::new();
        let mut params_refs = Vec::new();

        // INSERT INTO
        query.push_str(&format!("INSERT INTO {} ", self.table.as_ref().unwrap()));

        // Columns
        query.push_str(&format!("({}) ", self.columns.join(", ")));

        // VALUES
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
                params_refs.push(value);
                param_index += 1;
            }

            value_groups.push(format!("({})", placeholders.join(", ")));
        }

        query.push_str(&value_groups.join(", "));

        // RETURNING
        if let Some(returning) = &self.returning {
            query.push_str(&format!(" RETURNING {}", returning));
        }

        Ok((query, params_refs))
    }
}
