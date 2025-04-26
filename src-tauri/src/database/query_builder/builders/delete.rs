use crate::database::query_builder::{WhereBuilder, WhereClause};
use crate::{
    database::query_builder::{LogicalOperator, SqlQueryBuilder},
    database::{QueryBuilderError, QueryParam},
};

#[derive(Debug)]
pub struct DeleteQueryBuilder {
    table: Option<String>,
    where_clauses: Vec<WhereClause>,
    returning: Option<String>,
}

impl DeleteQueryBuilder {
    pub fn new() -> Self {
        DeleteQueryBuilder {
            table: None,
            where_clauses: Vec::new(),
            returning: None,
        }
    }

    pub fn table(mut self, table: &str) -> Self {
        self.table = Some(table.to_string());
        self
    }

    pub fn returning(mut self, returning: &str) -> Self {
        self.returning = Some(returning.to_string());
        self
    }
}

impl WhereBuilder for DeleteQueryBuilder {
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

impl SqlQueryBuilder for DeleteQueryBuilder {
    fn build(&self) -> Result<(String, Vec<&QueryParam>), QueryBuilderError> {
        if self.table.is_none() {
            return Err(QueryBuilderError::MissingTable);
        }

        let mut query = String::new();
        let mut params_refs = Vec::new();

        // DELETE FROM
        query.push_str(&format!("DELETE FROM {} ", self.table.as_ref().unwrap()));

        // WHERE
        let (where_clause, where_params, _) = Self::build_where_clause(&self.where_clauses, 1);

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
