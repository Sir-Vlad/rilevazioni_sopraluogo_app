use crate::database::query_builder::where_builder::{WhereBuilder, WhereClause};
use crate::{
    database::query_builder::{LogicalOperator, SqlQueryBuilder},
    database::{QueryBuilderError, QueryParam},
};

#[derive(Debug)]
pub struct UpdateQueryBuilder {
    table: Option<String>,
    set_clauses: Vec<(String, QueryParam)>,
    where_clauses: Vec<WhereClause>,
    returning: Option<String>,
}

impl UpdateQueryBuilder {
    pub fn new() -> Self {
        UpdateQueryBuilder {
            table: None,
            set_clauses: Vec::new(),
            where_clauses: Vec::new(),
            returning: None,
        }
    }

    pub fn table(mut self, table: &str) -> Self {
        self.table = Some(table.to_string());
        self
    }

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

    pub fn returning(mut self, returning: &str) -> Self {
        self.returning = Some(returning.to_string());
        self
    }
}

impl WhereBuilder for UpdateQueryBuilder {
    fn add_where_clause(&mut self, clause: WhereClause) {
        self.where_clauses.push(clause);
    }

    fn with_or(mut self) -> Self {
        if let Some(last) = self.where_clauses.last_mut() {
            last.operator = LogicalOperator::Or;
        }
        self
    }
}
impl SqlQueryBuilder for UpdateQueryBuilder {
    fn build(&self) -> Result<(String, Vec<&QueryParam>), QueryBuilderError> {
        if self.table.is_none() {
            return Err(QueryBuilderError::MissingTable);
        }

        if self.set_clauses.is_empty() {
            return Err(QueryBuilderError::MissingSetClauses);
        }

        let mut query = String::new();
        let mut params_refs = Vec::new();
        let mut param_index = 1;

        // UPDATE
        query.push_str(&format!("UPDATE {} SET ", self.table.as_ref().unwrap()));

        // SET clauses
        let mut set_parts = Vec::new();

        for (column, value) in &self.set_clauses {
            set_parts.push(format!("{} = ${}", column, param_index));
            params_refs.push(value);
            param_index += 1;
        }

        query.push_str(&set_parts.join(", "));

        // WHERE
        let (where_clause, where_params, _) =
            Self::build_where_clause(&self.where_clauses, param_index);

        if let Some(where_str) = where_clause {
            query.push(' ');
            query.push_str(&where_str);
            params_refs.extend(where_params);
        }


        // RETURNING
        if let Some(returning) = &self.returning {
            query.push_str(&format!(" RETURNING {}", returning));
        }

        Ok((query, params_refs))
    }
}
