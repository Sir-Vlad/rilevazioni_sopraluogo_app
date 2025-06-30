use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Command format invalid: missing '>' character at the start of the string")]
    MissingCommandPrefix,
    #[error("Command format invalid: missing 's' character at the start of the string")]
    MissingCommandStanzaPrefix,
    #[error("Command type is not supported: {0}")]
    UnknownCommandType(String),
    #[error("Invalid format: missing '->' character between stanze e campi")]
    MissingArrow,
    #[error("Empty stanze")]
    EmptyStanze,
    #[error("Empty campi")]
    EmptyFieldUpdates,
    #[error("Malformed input of the field: {0}")]
    MalformedFieldUpdate(String),
    #[error("Finding empty stanza")]
    EmptyStanza,
    #[error("Finding empty fields")]
    EmptyFields,
    #[error("Error generic: {0}")]
    Generic(String),
}
