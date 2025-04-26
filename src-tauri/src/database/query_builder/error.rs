#[derive(Debug)]
pub enum QueryBuilderError {
    MissingTable,
    MissingColumns,
    MissingValues,
    ColumnValueMismatch,
    MissingSetClauses,
}

impl std::fmt::Display for QueryBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingTable => write!(f, "Missing table name"),
            Self::MissingColumns => write!(f, "Missing columns"),
            Self::MissingValues => write!(f, "Missing values"),
            Self::ColumnValueMismatch => write!(f, "Column value mismatch"),
            Self::MissingSetClauses => write!(f, "Missing SET clauses"),
        }
    }
}

impl std::error::Error for QueryBuilderError {}
