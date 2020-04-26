use std::collections::HashMap;

type JsonValues = Vec<JsonValue>;
type JsonObject = HashMap<String, JsonValue>;

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Number(f64),
    Bool(bool),
    String(String),
    Array(JsonValues),
    Object(JsonObject),
    Null,
}

// TODO parser部分は後で別モジュールに書く
//
use std::fmt;

pub type ParseResult = Result<JsonValue, ParseError>;

#[derive(Debug)]
pub struct ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

impl std::error::Error for ParseError {}

// TODO iteratorを取ったほうが汎用的なり
pub struct JsonParser {
    chars: Vec<char>,
}
