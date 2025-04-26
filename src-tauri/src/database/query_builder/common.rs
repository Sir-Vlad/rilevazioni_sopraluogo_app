use crate::database::{QueryBuilderError, QueryParam};

pub trait SqlQueryBuilder {
    fn build(&self) -> Result<(String, Vec<&QueryParam>), QueryBuilderError>;
}

#[derive(Debug)]
pub(crate) enum LogicalOperator {
    And,
    Or,
}

impl Default for LogicalOperator {
    fn default() -> Self {
        Self::And
    }
}

impl std::fmt::Display for LogicalOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::And => write!(f, "AND"),
            Self::Or => write!(f, "OR"),
        }
    }
}
