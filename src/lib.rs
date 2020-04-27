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

pub struct JsonParser<C: Iterator<Item = char>> {
    chars: C,
    col: usize,
    line: usize,
}

impl<C> JsonParser<C>
where
    C: Iterator<Item = char>,
{
    // NOTE Iterator traitとしても良いかもしれない
    fn netx(&mut self) -> Option<char> {
        while let Some(c) = self.chars.next() {
            // skip beakline
            if c == '\n' {
                self.col = 0;
                self.line += 1;
            }

            self.col += 1;
            if !c.is_whitespace() {
                return Some(c);
            }
        }

        None
    }
}
