use rusqlite::ToSql;

pub trait SqlParams {
    fn to_sql_params(&self) -> Vec<&dyn ToSql>;
}

// Tipi interi con segno
impl SqlParams for i8 {
    fn to_sql_params(&self) -> Vec<&dyn ToSql> {
        vec![self]
    }
}

impl SqlParams for i16 {
    fn to_sql_params(&self) -> Vec<&dyn ToSql> {
        vec![self]
    }
}

impl SqlParams for i32 {
    fn to_sql_params(&self) -> Vec<&dyn ToSql> {
        vec![self]
    }
}

impl SqlParams for i64 {
    fn to_sql_params(&self) -> Vec<&dyn ToSql> {
        vec![self]
    }
}

impl SqlParams for isize {
    fn to_sql_params(&self) -> Vec<&dyn ToSql> {
        vec![self]
    }
}

// Tipi interi senza segno
impl SqlParams for u8 {
    fn to_sql_params(&self) -> Vec<&dyn ToSql> {
        vec![self]
    }
}

impl SqlParams for u16 {
    fn to_sql_params(&self) -> Vec<&dyn ToSql> {
        vec![self]
    }
}

impl SqlParams for u32 {
    fn to_sql_params(&self) -> Vec<&dyn ToSql> {
        vec![self]
    }
}

impl SqlParams for u64 {
    fn to_sql_params(&self) -> Vec<&dyn ToSql> {
        vec![self]
    }
}

impl SqlParams for usize {
    fn to_sql_params(&self) -> Vec<&dyn ToSql> {
        vec![self]
    }
}

// Tipi floating-point
impl SqlParams for f32 {
    fn to_sql_params(&self) -> Vec<&dyn ToSql> {
        vec![self]
    }
}

impl SqlParams for f64 {
    fn to_sql_params(&self) -> Vec<&dyn ToSql> {
        vec![self]
    }
}

// Tipo booleano
impl SqlParams for bool {
    fn to_sql_params(&self) -> Vec<&dyn ToSql> {
        vec![self]
    }
}

// Tipi stringa
impl SqlParams for String {
    fn to_sql_params(&self) -> Vec<&dyn ToSql> {
        vec![self]
    }
}

impl SqlParams for &str {
    fn to_sql_params(&self) -> Vec<&dyn ToSql> {
        vec![self]
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
}
