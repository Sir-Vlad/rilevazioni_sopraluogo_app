use crate::service::parser::{
    command::{Parser, TypeValue},
    error::ParserError,
};

pub struct StanzaParser;

impl Parser<StanzaUpdate> for StanzaParser {
    fn parse(input: &str) -> Result<StanzaUpdate, ParserError> {
        if !input.starts_with("s") {
            return Err(ParserError::MissingCommandStanzaPrefix);
        }
        let input = &input[1..];
        let split: Vec<&str> = input.split("->").collect();
        if split.len() != 2 {
            return Err(ParserError::MissingArrow);
        }

        let stanze = split
            .first()
            .unwrap()
            .split(",")
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();
        let fields = Self::split_fields(split.get(1).unwrap());
        let fields_updated: Vec<FieldStanzaUpdate> = fields
            .iter()
            .map(|x| x.split_once(":").unwrap())
            .map(|(x, y)| FieldStanzaUpdate::new(x, y))
            .collect();

        Ok(StanzaUpdate::new(stanze, fields_updated))
    }
}

impl StanzaParser {
    fn split_fields(input: &str) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let mut current = String::new();
        let chars = input.chars();
        let mut trigger = false;

        for c in chars {
            match c {
                ',' => {
                    if trigger {
                        current.push(c);
                    } else {
                        result.push(current.clone());
                        current.clear();
                    }
                }
                '{' => {
                    trigger = true;
                    current.push(c);
                }
                '}' => {
                    trigger = false;
                    current.push(c);
                    result.push(current.clone());
                    current.clear();
                }
                _ => current.push(c),
            }
        }

        if !current.is_empty() {
            result.push(current);
        }
        result.iter().map(|s| s.trim().to_string()).collect()
    }
}

#[derive(Debug, Default)]
pub struct StanzaUpdate {
    pub stanze: Vec<String>,
    pub fields_updates: Vec<FieldStanzaUpdate>,
}

impl StanzaUpdate {
    pub fn new(stanze: Vec<String>, fields: Vec<FieldStanzaUpdate>) -> Self {
        Self {
            stanze,
            fields_updates: fields,
        }
    }
}

#[derive(Debug, Default)]
pub struct FieldStanzaUpdate {
    pub field: TypeFieldStanza,
    pub value: TypeValue,
}

impl FieldStanzaUpdate {
    pub fn new(field: &str, value: &str) -> Self {
        Self {
            field: TypeFieldStanza::try_from(field).unwrap(),
            value: TypeValue::try_from(value).unwrap(),
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum TypeFieldStanza {
    Illuminazione,
    Riscaldamento,
    Raffrescamento,
    Infissi,
    SpessoreMuro,
    #[default]
    Altezza,
}
impl TryFrom<&str> for TypeFieldStanza {
    type Error = ParserError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "i" => Ok(TypeFieldStanza::Illuminazione),
            "r" => Ok(TypeFieldStanza::Riscaldamento),
            "R" => Ok(TypeFieldStanza::Raffrescamento),
            "I" => Ok(TypeFieldStanza::Infissi),
            "a" => Ok(TypeFieldStanza::Altezza),
            "s" => Ok(TypeFieldStanza::SpessoreMuro),
            _ => Err(ParserError::Generic("Campo non esistente".to_string())),
        }
    }
}
