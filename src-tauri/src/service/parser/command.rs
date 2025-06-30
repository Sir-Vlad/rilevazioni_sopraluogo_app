use crate::service::parser::{
    error::ParserError,
    stanza_parser::{StanzaParser, StanzaUpdate},
};
use std::collections::HashMap;
use std::ops::Deref;

pub trait Parser<T> {
    fn parse(input: &str) -> Result<T, ParserError>;
}

#[derive(Debug)]
pub enum Command {
    Internal(InternalCommand),
}

impl Parser<Command> for Command {
    fn parse(input: &str) -> Result<Command, ParserError> {
        if input.trim().starts_with(">") {
            Ok(Command::Internal(InternalCommand::parse(&input[1..])?))
        } else {
            Err(ParserError::UnknownCommandType(
                input[0..1].trim().to_string(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum InternalCommand {
    Stanza(StanzaUpdate),
    Infisso(),
}

impl Parser<InternalCommand> for InternalCommand {
    fn parse(input: &str) -> Result<InternalCommand, ParserError> {
        if input.trim().starts_with("s") {
            let stanza_update = StanzaParser::parse(input)?;
            Ok(InternalCommand::Stanza(stanza_update))
        } else if input.trim().starts_with("i") {
            Ok(InternalCommand::Infisso())
        } else {
            Err(ParserError::UnknownCommandType(
                input[0..1].trim().to_string(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum TypeValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Object(HashMap<String, TypeValue>),
    Array(Vec<TypeValue>),
    Null(Option<Box<TypeValue>>),
}

impl Default for TypeValue {
    fn default() -> Self {
        TypeValue::Null(None)
    }
}

impl TryFrom<&str> for TypeValue {
    type Error = ParserError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            _ if value.starts_with('{') && value.ends_with('}') => {
                let mut hash_map: HashMap<String, TypeValue> = HashMap::new();
                let values = value[1..value.len() - 1].split(",").collect::<Vec<&str>>();

                for value in values {
                    let x = value.trim().split(":").collect::<Vec<&str>>();
                    if x.len() != 2 {
                        return Err(ParserError::Generic(
                            "The values must have two colons".to_string(),
                        ));
                    }
                    let key = x.first().unwrap();
                    let value = x.get(1).unwrap();
                    hash_map.insert(key.to_string(), TypeValue::try_from(*value)?);
                }
                Ok(TypeValue::Object(hash_map))
            }
            _ if value.contains("false") || value.contains("true") => Ok(TypeValue::Boolean(
                value
                    .parse::<bool>()
                    .map_err(|e| ParserError::Generic(e.to_string()))?,
            )),
            _ if value.parse::<i64>().is_ok() => Ok(TypeValue::Integer(
                value
                    .parse::<i64>()
                    .map_err(|e| ParserError::Generic(e.to_string()))?,
            )),
            _ if value.parse::<f64>().is_ok() => Ok(TypeValue::Float(
                value
                    .parse::<f64>()
                    .map_err(|e| ParserError::Generic(e.to_string()))?,
            )),
            _ => Ok(TypeValue::String(value.to_string())),
        }
    }
}

impl TryFrom<&TypeValue> for String {
    type Error = ParserError;

    fn try_from(value: &TypeValue) -> Result<Self, Self::Error> {
        if let TypeValue::String(value) = value {
            Ok(value.clone())
        } else {
            Err(ParserError::Generic("Expected a string".to_string()))
        }
    }
}

impl TryFrom<&TypeValue> for i64 {
    type Error = ParserError;

    fn try_from(value: &TypeValue) -> Result<Self, Self::Error> {
        if let TypeValue::Integer(value) = value {
            Ok(*value)
        } else {
            Err(ParserError::Generic("Expected a i64".to_string()))
        }
    }
}

impl TryFrom<&TypeValue> for f64 {
    type Error = ParserError;

    fn try_from(value: &TypeValue) -> Result<Self, Self::Error> {
        if let TypeValue::Float(value) = value {
            Ok(*value)
        } else {
            Err(ParserError::Generic("Expected a f64".to_string()))
        }
    }
}

impl<'a> TryFrom<&'a TypeValue> for &'a HashMap<String, TypeValue> {
    type Error = ParserError;

    fn try_from(value: &'a TypeValue) -> Result<Self, Self::Error> {
        match value {
            TypeValue::Object(value) => Ok(value),
            _ => Err(ParserError::Generic("Expected a HashMap".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::parser::TypeFieldStanza::{
        Illuminazione, Infissi, Raffrescamento, Riscaldamento,
    };

    #[test]
    fn test_parses_stanza() {
        let string_parsed = ">s 258, 845 , 426 -> i:l,r:t,R:s, I:{A:3,B:8}";
        let res = Command::parse(string_parsed).unwrap();
        println!("{:#?}", res);
    }

    #[test]
    fn test_multiple_stanza() {
        let string_parsed = ">s 258,458,125 -> i:l";
        let res = Command::parse(string_parsed).unwrap();
        if let Command::Internal(InternalCommand::Stanza(stanza)) = res {
            assert_eq!(3, stanza.stanze.len());
            assert_eq!(stanza.stanze, vec!["258", "458", "125"]);
        } else {
            panic!("Wrong result type");
        }
    }

    #[test]
    fn test_multiple_fields() {
        let string_parsed = ">s 258 -> i:l,r:t,R:s,I:{A:3,B:8}";
        let res = Command::parse(string_parsed).unwrap();
        if let Command::Internal(InternalCommand::Stanza(stanza)) = res {
            assert_eq!(4, stanza.fields_updates.len());
            assert_eq!(stanza.fields_updates[0].field, Illuminazione);
            assert_eq!(stanza.fields_updates[1].field, Riscaldamento);
            assert_eq!(stanza.fields_updates[2].field, Raffrescamento);
            assert_eq!(stanza.fields_updates[3].field, Infissi);
        } else {
            panic!("Wrong result type");
        }
    }
}
