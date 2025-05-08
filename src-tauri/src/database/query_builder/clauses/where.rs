use crate::database::query_builder::LogicalOperator;
use crate::database::QueryParam;

#[derive(Debug)]
pub struct WhereClause {
    pub(crate) condition: String,
    pub(crate) params: Vec<QueryParam>,
    pub(crate) operator: LogicalOperator,
}

pub trait WhereBuilder {
    fn add_where_clause(&mut self, clause: WhereClause);
    fn with_or(self) -> Self
    where
        Self: Sized;

    fn where_raw(mut self, condition: &str) -> Self
    where
        Self: Sized,
    {
        self.add_where_clause(WhereClause {
            condition: condition.to_string(),
            params: Vec::new(),
            operator: LogicalOperator::And,
        });
        self
    }

    fn where_eq<T: Into<QueryParam>>(mut self, column: &str, value: T) -> Self
    where
        Self: Sized,
    {
        let param = value.into();
        self.add_where_clause(WhereClause {
            condition: format!("{} = ?", column),
            params: vec![param],
            operator: LogicalOperator::And,
        });
        self
    }

    fn where_not_eq<T: Into<QueryParam>>(mut self, column: &str, value: T) -> Self
    where
        Self: Sized,
    {
        let param = value.into();
        self.add_where_clause(WhereClause {
            condition: format!("{} != ?", column),
            params: vec![param],
            operator: LogicalOperator::And,
        });
        self
    }

    fn where_gt<T: Into<QueryParam>>(mut self, column: &str, value: T) -> Self
    where
        Self: Sized,
    {
        let param = value.into();
        self.add_where_clause(WhereClause {
            condition: format!("{} > ?", column),
            params: vec![param],
            operator: LogicalOperator::And,
        });
        self
    }

    fn where_gte<T: Into<QueryParam>>(mut self, column: &str, value: T) -> Self
    where
        Self: Sized,
    {
        let param = value.into();
        self.add_where_clause(WhereClause {
            condition: format!("{} >= ?", column),
            params: vec![param],
            operator: LogicalOperator::And,
        });
        self
    }

    fn where_lt<T: Into<QueryParam>>(mut self, column: &str, value: T) -> Self
    where
        Self: Sized,
    {
        let param = value.into();
        self.add_where_clause(WhereClause {
            condition: format!("{} < ?", column),
            params: vec![param],
            operator: LogicalOperator::And,
        });
        self
    }

    fn where_lte<T: Into<QueryParam>>(mut self, column: &str, value: T) -> Self
    where
        Self: Sized,
    {
        let param = value.into();
        self.add_where_clause(WhereClause {
            condition: format!("{} <= ?", column),
            params: vec![param],
            operator: LogicalOperator::And,
        });
        self
    }

    fn where_like<T: Into<QueryParam>>(mut self, column: &str, value: T) -> Self
    where
        Self: Sized,
    {
        let param = value.into();
        self.add_where_clause(WhereClause {
            condition: format!("{} LIKE ?", column),
            params: vec![param],
            operator: LogicalOperator::And,
        });
        self
    }

    fn where_not_like<T: Into<QueryParam>>(mut self, column: &str, value: T) -> Self
    where
        Self: Sized,
    {
        let param = value.into();
        self.add_where_clause(WhereClause {
            condition: format!("{} NOT LIKE ?", column),
            params: vec![param],
            operator: LogicalOperator::And,
        });
        self
    }

    fn where_in<T: Into<QueryParam>>(mut self, column: &str, values: Vec<T>) -> Self
    where
        Self: Sized,
    {
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

        self.add_where_clause(WhereClause {
            condition: format!("{} IN ({})", column, placeholders),
            params,
            operator: LogicalOperator::And,
        });
        self
    }

    fn where_not_in<T: Into<QueryParam>>(mut self, column: &str, values: Vec<T>) -> Self
    where
        Self: Sized,
    {
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

        self.add_where_clause(WhereClause {
            condition: format!("{} NOT IN ({})", column, placeholders),
            params,
            operator: LogicalOperator::And,
        });
        self
    }

    fn where_is_null(mut self, column: &str) -> Self
    where
        Self: Sized,
    {
        self.add_where_clause(WhereClause {
            condition: format!("{} IS NULL", column),
            params: Vec::new(),
            operator: LogicalOperator::And,
        });
        self
    }

    fn where_is_not_null(mut self, column: &str) -> Self
    where
        Self: Sized,
    {
        self.add_where_clause(WhereClause {
            condition: format!("{} IS NOT NULL", column),
            params: Vec::new(),
            operator: LogicalOperator::And,
        });
        self
    }

    fn build_where_clause(
        where_clauses: &[WhereClause],
        param_start_index: usize,
    ) -> (Option<String>, Vec<&QueryParam>, usize) {
        if where_clauses.is_empty() {
            return (None, Vec::new(), param_start_index);
        }

        let mut where_query = String::from("WHERE ");
        let mut params_refs = Vec::new();
        let mut first = true;
        let mut current_param_index = param_start_index;

        for clause in where_clauses {
            if !first {
                where_query.push_str(&format!(" {} ", clause.operator));
            } else {
                first = false;
            }

            let mut clause_text = clause.condition.clone();
            while let Some(pos) = clause_text.find('?') {
                clause_text = clause_text[..pos].to_string()
                    + &format!("${}", current_param_index)
                    + &clause_text[pos + 1..];
                current_param_index += 1;
            }

            where_query.push_str(&clause_text);

            for param in &clause.params {
                params_refs.push(param);
            }
        }

        (Some(where_query), params_refs, current_param_index)
    }
}
