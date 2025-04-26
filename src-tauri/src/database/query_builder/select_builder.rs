use crate::database::query_builder::where_builder::{WhereBuilder, WhereClause};
use crate::{
    database::query_builder::{LogicalOperator, SqlQueryBuilder},
    database::{QueryBuilderError, QueryParam},
};

pub struct SelectQueryBuilder {
    table: Option<String>,
    selected_columns: Option<Vec<String>>,
    where_clauses: Vec<WhereClause>,
    joins: Vec<JoinClause>,
    order_by: Option<String>,
    limit: Option<usize>,
    offset: Option<usize>,
    group_by: Option<Vec<String>>,
    having: Option<String>,
}

struct JoinClause {
    join_type: JoinType,
    table: String,
    condition: String,
}

enum JoinType {
    Inner,
    Left,
    Right,
    Full,
}

impl SelectQueryBuilder {
    pub fn new() -> Self {
        SelectQueryBuilder {
            table: None,
            selected_columns: None,
            where_clauses: Vec::new(),
            joins: Vec::new(),
            order_by: None,
            limit: None,
            offset: None,
            group_by: None,
            having: None,
        }
    }

    pub fn table(mut self, table: &str) -> Self {
        self.table = Some(table.to_string());
        self
    }

    pub fn columns(mut self, columns: Vec<&str>) -> Self {
        self.selected_columns = Some(columns.into_iter().map(|c| c.to_string()).collect());
        self
    }
    pub fn inner_join(mut self, table: &str, condition: &str) -> Self {
        self.joins.push(JoinClause {
            join_type: JoinType::Inner,
            table: table.to_string(),
            condition: condition.to_string(),
        });
        self
    }

    pub fn left_join(mut self, table: &str, condition: &str) -> Self {
        self.joins.push(JoinClause {
            join_type: JoinType::Left,
            table: table.to_string(),
            condition: condition.to_string(),
        });
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

    pub fn group_by(mut self, columns: Vec<&str>) -> Self {
        self.group_by = Some(columns.into_iter().map(|c| c.to_string()).collect());
        self
    }

    pub fn having(mut self, condition: &str) -> Self {
        self.having = Some(condition.to_string());
        self
    }
}

impl WhereBuilder for SelectQueryBuilder {
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

impl SqlQueryBuilder for SelectQueryBuilder {
    fn build(&self) -> Result<(String, Vec<&QueryParam>), QueryBuilderError> {
        if self.table.is_none() {
            return Err(QueryBuilderError::MissingTable);
        }

        let mut query = String::new();
        let mut params_refs = Vec::new();

        // SELECT
        query.push_str("SELECT ");
        if let Some(columns) = &self.selected_columns {
            query.push_str(&columns.join(", "));
        } else {
            query.push('*');
        }

        // FROM
        query.push_str(" FROM ");
        query.push_str(self.table.as_ref().unwrap());

        // JOINs
        for join in &self.joins {
            match join.join_type {
                JoinType::Inner => query.push_str(" INNER JOIN "),
                JoinType::Left => query.push_str(" LEFT JOIN "),
                JoinType::Right => query.push_str(" RIGHT JOIN "),
                JoinType::Full => query.push_str(" FULL JOIN "),
            }
            query.push_str(&join.table);
            query.push_str(" ON ");
            query.push_str(&join.condition);
        }

        // WHERE
        let (where_clause, where_params, _) = Self::build_where_clause(&self.where_clauses, 1);

        if let Some(where_str) = where_clause {
            query.push(' ');
            query.push_str(&where_str);
            params_refs.extend(where_params);
        }

        // GROUP BY
        if let Some(group_by) = &self.group_by {
            query.push_str(" GROUP BY ");
            query.push_str(&group_by.join(", "));
        }

        // HAVING
        if let Some(having) = &self.having {
            query.push_str(" HAVING ");
            query.push_str(having);
        }

        // ORDER BY
        if let Some(order_by) = &self.order_by {
            query.push_str(" ORDER BY ");
            query.push_str(order_by);
        }

        // LIMIT
        if let Some(limit) = self.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        // OFFSET
        if let Some(offset) = self.offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }

        Ok((query, params_refs))
    }
}
