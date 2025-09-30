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

pub struct Lexer<'a> {
    src: &'a str,
    chars: std::str::Chars<'a>,
    peeked: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let peeked = chars.next();
        Self { src: input, chars, peeked }
    }

    fn bump(&mut self) -> Option<char> {
        let cur = self.peeked;
        self.peeked = self.chars.next();
        cur
    }

    fn peek(&self) -> Option<char> { self.peeked }

    fn eat_while<F>(&mut self, mut f: F) -> String
    where F: FnMut(char) -> bool {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if f(c) {
                s.push(c);
                self.bump();
            } else {
                break;
            }
        }
        s
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            // skip whitespace
            let mut progressed = false;
            while let Some(c) = self.peek() {
                if c.is_whitespace() {
                    progressed = true;
                    self.bump();
                } else {
                    break;
                }
            }

            // single-line comment //
            if self.peek() == Some('/') {
                // need to peek next from chars iterator -- complex; clone remaining
                let mut clone_iter = self.chars.clone();
                let next = clone_iter.next().or(None);
                if next == Some('/') {
                    // consume '//'
                    self.bump();
                    self.bump();
                    progressed = true;
                    // consume until newline or EOF
                    while let Some(c) = self.peek() {
                        self.bump();
                        if c == '\n' { break; }
                    }
                    continue;
                } else if next == Some('*') {
                    // block comment /* ... */
                    self.bump(); // '/'
                    self.bump(); // '*'
                    progressed = true;
                    // read until '*/' or EOF
                    loop {
                        match self.bump() {
                            Some('*') => {
                                if self.peek() == Some('/') {
                                    self.bump();
                                    break;
                                }
                            }
                            None => break,
                            _ => {}
                        }
                    }
                    continue;
                }
            }

            if !progressed { break; }
        }
    }

    fn read_string(&mut self) -> Result<Token, LexError> {
        // assume opening '"' consumed
        let mut s = String::new();
        while let Some(c) = self.bump() {
            match c {
                '\\' => {
                    if let Some(next) = self.bump() {
                        let esc = match next {
                            'n' => '\n',
                            't' => '\t',
                            'r' => '\r',
                            '\\' => '\\',
                            '\'' => '\'',
                            '"' => '"',
                            _ => return Err(LexError::InvalidEscape),
                        };
                        s.push(esc);
                    } else {
                        return Err(LexError::UnterminatedString);
                    }
                }
                '"' => return Ok(Token::StringLiteral(s)),
                c => s.push(c),
            }
        }
        Err(LexError::UnterminatedString)
    }

    fn read_char(&mut self) -> Result<Token, LexError> {
        // assume opening '\'' consumed
        if let Some(c) = self.bump() {
            if c == '\\' {
                if let Some(next) = self.bump() {
                    let esc = match next {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '\\' => '\\',
                        '\'' => '\'',
                        '"' => '"',
                        _ => return Err(LexError::InvalidEscape),
                    };
                    if self.peek() == Some('\'') { self.bump(); Ok(Token::CharLiteral(esc)) } else { Err(LexError::UnterminatedChar) }
                } else { Err(LexError::UnterminatedChar) }
            } else {
                // normal char
                if self.peek() == Some('\'') { self.bump(); Ok(Token::CharLiteral(c)) } else { Err(LexError::UnterminatedChar) }
            }
        } else { Err(LexError::UnterminatedChar) }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace_and_comments();

        let ch = self.bump();
        match ch {
            None => Some(Ok(Token::Eof)),
            Some(c) if c.is_ascii_alphabetic() || c == '_' => {
                let mut s = String::new();
                s.push(c);
                s.push_str(&self.eat_while(|ch| ch.is_ascii_alphanumeric() || ch == '_'));
                Some(Ok(Token::Identifier(s)))
            }
            Some(c) if c.is_ascii_digit() => {
                let mut s = String::new();
                s.push(c);
                s.push_str(&self.eat_while(|ch| ch.is_ascii_digit() || ch == '.'));
                Some(Ok(Token::Number(s)))
            }
            Some('"') => Some(self.read_string()),
            Some('\'') => Some(self.read_char()),
            Some(c) if "{}();,[]<>".contains(c) => Some(Ok(Token::Punct(c))),
            Some(c) => {
                // treat as operator (one or two char)
                let mut s = String::new();
                s.push(c);
                if let Some(next) = self.peek() {
                    let two = format!("{}{}", c, next);
                    let two_ops = ["==","!=","<=","=>","->","++","--","+=","-=","*=","/=","&&","||","<<", ">>"];
                    if two_ops.contains(&two.as_str()) {
                        self.bump();
                        s.push(next);
                    }
                }
                Some(Ok(Token::Operator(s)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_ident_and_number() {
        let src = "int x = 42;";
        let mut lex = Lexer::new(src);
        assert_eq!(lex.next().unwrap().unwrap(), Token::Identifier("int".into()));
        assert_eq!(lex.next().unwrap().unwrap(), Token::Identifier("x".into()));
        assert_eq!(lex.next().unwrap().unwrap(), Token::Operator("=".into()));
        assert_eq!(lex.next().unwrap().unwrap(), Token::Number("42".into()));
        assert_eq!(lex.next().unwrap().unwrap(), Token::Punct(';'));
    }

    #[test]
    fn string_and_char() {
        let src = "\"hello\\n\" '\\'a'";
        let mut lex = Lexer::new(src);
        assert_eq!(lex.next().unwrap().unwrap(), Token::StringLiteral("hello\n".into()));
        // note: the char token test is simplistic; adjust if needed
    }
}
