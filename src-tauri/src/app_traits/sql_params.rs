use crate::database::QueryParam;
use rusqlite::ToSql;

pub trait SqlParams {
    fn to_sql_params(&self) -> Vec<&dyn ToSql>;
    fn params_count() -> usize
    where
        Self: Sized;
}

// Tipi interi con segno
impl SqlParams for i8 {
    fn to_sql_params(&self) -> Vec<&dyn ToSql> {
        vec![self]
    }

    fn params_count() -> usize {
        1
    }
}

impl SqlParams for i16 {
    fn to_sql_params(&self) -> Vec<&dyn ToSql> {
        vec![self]
    }

    fn params_count() -> usize {
        1
    }
}

impl SqlParams for i32 {
    fn to_sql_params(&self) -> Vec<&dyn ToSql> {
        vec![self]
    }

    fn params_count() -> usize {
        1
    }
}

impl SqlParams for i64 {
    fn to_sql_params(&self) -> Vec<&dyn ToSql> {
        vec![self]
    }

    fn params_count() -> usize {
        1
    }
}

impl SqlParams for isize {
    fn to_sql_params(&self) -> Vec<&dyn ToSql> {
        vec![self]
    }

    fn params_count() -> usize {
        1
    }
}

// Tipi interi senza segno
impl SqlParams for u8 {
    fn to_sql_params(&self) -> Vec<&dyn ToSql> {
        vec![self]
    }

    fn params_count() -> usize {
        1
    }
}

impl SqlParams for u16 {
    fn to_sql_params(&self) -> Vec<&dyn ToSql> {
        vec![self]
    }

    fn params_count() -> usize {
        1
    }
}

impl SqlParams for u32 {
    fn to_sql_params(&self) -> Vec<&dyn ToSql> {
        vec![self]
    }

    fn params_count() -> usize {
        1
    }
}

impl SqlParams for u64 {
    fn to_sql_params(&self) -> Vec<&dyn ToSql> {
        vec![self]
    }

    fn params_count() -> usize {
        1
    }
}

impl SqlParams for usize {
    fn to_sql_params(&self) -> Vec<&dyn ToSql> {
        vec![self]
    }

    fn params_count() -> usize {
        1
    }
}

// Tipi floating-point
impl SqlParams for f32 {
    fn to_sql_params(&self) -> Vec<&dyn ToSql> {
        vec![self]
    }

    fn params_count() -> usize {
        1
    }
}

impl SqlParams for f64 {
    fn to_sql_params(&self) -> Vec<&dyn ToSql> {
        vec![self]
    }

    fn params_count() -> usize {
        1
    }
}

// Tipo booleano
impl SqlParams for bool {
    fn to_sql_params(&self) -> Vec<&dyn ToSql> {
        vec![self]
    }

    fn params_count() -> usize {
        1
    }
}

// Tipi stringa
impl SqlParams for String {
    fn to_sql_params(&self) -> Vec<&dyn ToSql> {
        vec![self]
    }

    fn params_count() -> usize {
        1
    }
}

impl SqlParams for &str {
    fn to_sql_params(&self) -> Vec<&dyn ToSql> {
        vec![self]
    }

    fn params_count() -> usize {
        1
    }
}

impl<T> SqlParams for Option<T>
where
    T: ToSql,
{
    fn to_sql_params(&self) -> Vec<&dyn ToSql> {
        if let Some(value) = self {
            vec![value as &dyn ToSql]
        } else {
            vec![&rusqlite::types::Null as &dyn ToSql]
        }
    }

    fn params_count() -> usize
    where
        Self: Sized,
    {
        1
    }
}

// Implementazione generica per tuple
impl<T1, T2> SqlParams for (T1, T2)
where
    T1: ToSql,
    T2: ToSql,
{
    fn to_sql_params(&self) -> Vec<&dyn ToSql> {
        vec![&self.0, &self.1]
    }

    fn params_count() -> usize {
        2
    }
}

impl SqlParams for QueryParam {
    fn to_sql_params(&self) -> Vec<&dyn ToSql> {
        match self {
            QueryParam::String(val) => vec![val],
            QueryParam::Integer(val) => vec![val],
            QueryParam::Float(val) => vec![val],
            QueryParam::Boolean(val) => vec![val],
            QueryParam::Null => vec![&rusqlite::types::Null],
        }
    }

    fn params_count() -> usize {
        1 // Ogni `QueryParam` rappresenta un singolo valore
    }
}
