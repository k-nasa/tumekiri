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
use std::fmt;
use std::iter::Peekable;

pub type ParseResult = Result<JsonValue, ParseError>;

#[derive(Debug)]
pub struct ParseError {
    msg: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for ParseError {}

pub struct JsonParser<C: Iterator<Item = char>> {
    chars: Peekable<C>,
    col: usize,
    line: usize,
}

impl<C> JsonParser<C>
where
    C: Iterator<Item = char>,
{
    pub fn new(input: C) -> Self {
        JsonParser {
            chars: input.peekable(),
            col: 0,
            line: 0,
        }
    }

    pub fn parse(&mut self) -> ParseResult {
        let first_char = match self.peek() {
            None => return self.error_result("Invalid input"),
            Some(c) => c,
        };

        match first_char {
            '"' => self.parse_string(),
            '0'..='9' => self.parse_number(),
            '{' => self.parse_object(),
            '[' => self.parse_array(),
            't' | 'f' => self.parse_bool(),
            'n' => self.parse_null(),
            c => self.error_result(&format!("Unsupported charactor {}", c)),
        }
    }

    pub fn parse_string(&mut self) -> ParseResult {
        if self.chars.next() != Some('"') {
            return self.error_result("");
        }

        let mut output = String::new();

        loop {
            let c = match self.chars.next() {
                Some('"') => break,
                None => return self.error_result(""),

                Some(c) => c,
            };

            output.push(c)
        }

        Ok(JsonValue::String(output))
    }

    pub fn parse_number(&mut self) -> ParseResult {
        todo!()
    }

    pub fn parse_array(&mut self) -> ParseResult {
        todo!()
    }

    pub fn parse_object(&mut self) -> ParseResult {
        todo!()
    }

    pub fn parse_bool(&mut self) -> ParseResult {
        todo!()
    }

    pub fn parse_null(&mut self) -> ParseResult {
        todo!()
    }

    fn peek(&mut self) -> Option<char> {
        while let Some(&c) = self.chars.peek() {
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

    fn error_result(&self, msg: &str) -> ParseResult {
        Err(self.make_error(msg))
    }

    fn make_error(&self, msg: &str) -> ParseError {
        ParseError {
            msg: String::from(msg),
        }
    }
}

impl<C> std::iter::Iterator for JsonParser<C>
where
    C: Iterator<Item = char>,
{
    type Item = char;

    fn next(&mut self) -> Option<<Self as std::iter::Iterator>::Item> {
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
