use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(String),
    StringLiteral(String),
    CharLiteral(char),
    Operator(String),
    Punct(char),
    Eof,
}

#[derive(Debug)]
pub enum LexError {
    UnterminatedString,
    UnterminatedChar,
    InvalidEscape,
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexError::UnterminatedString => write!(f, "unterminated string literal"),
            LexError::UnterminatedChar => write!(f, "unterminated char literal"),
            LexError::InvalidEscape => write!(f, "invalid escape sequence"),
        }
    }
}

impl std::error::Error for LexError {}

pub type LexResult<T> = Result<T, LexError>;
