#[derive(Debug)]
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
impl From<f32> for QueryParam {
    fn from(value: f32) -> Self {
        Self::Float(value as f64)
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

impl<T> From<Option<T>> for QueryParam
where
    T: Into<QueryParam>,
{
    fn from(value: Option<T>) -> Self {
        match value {
            Some(val) => val.into(),
            None => QueryParam::Null,
        }
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
