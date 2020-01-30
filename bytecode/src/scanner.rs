use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error,
    EOF,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // We display enums as integer values as it were a C-like enum.
        write!(f, "{}", *self as i32)
    }
}

pub struct Token<'b> {
    pub typ: TokenType,
    pub src: &'b str,
    pub line: i32,
}

pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: i32,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source: source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    /// Return the character at the given position.
    ///
    /// This panics if the given position does not point to a valid char.
    /// This method is copied from the [Rust Regex parse](https://github.com/rust-lang/regex/blob/master/regex-syntax/src/ast/parse.rs#L461).
    fn char_at(&self, i: usize) -> char {
        self.source[i..]
            .chars()
            .next()
            .unwrap_or_else(|| panic!("expected char at offset {}", i))
    }

    fn is_at_end(&self) -> bool {
        self.char_at(self.current) == '\0'
    }

    fn advance(&mut self) -> char {
        self.current += 1;

        self.char_at(self.current - 1)
    }

    fn make_token(&self, typ: TokenType) -> Token {
        Token {
            typ: typ,
            src: &self.source[self.start..self.current],
            line: self.line,
        }
    }

    fn error_token(&self, message: &'static str) -> Token {
        Token {
            typ: TokenType::Error,
            src: message,
            line: self.line,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::EOF);
        }

        let c: char = self.advance();

        match c {
            '(' => return self.make_token(TokenType::LeftParen),
            ')' => return self.make_token(TokenType::RightParen),
            '{' => return self.make_token(TokenType::LeftBrace),
            '}' => return self.make_token(TokenType::RightBrace),
            ';' => return self.make_token(TokenType::Semicolon),
            ',' => return self.make_token(TokenType::Comma),
            '.' => return self.make_token(TokenType::Dot),
            '-' => return self.make_token(TokenType::Minus),
            '+' => return self.make_token(TokenType::Plus),
            '/' => return self.make_token(TokenType::Slash),
            '*' => return self.make_token(TokenType::Star),
            _ => unimplemented!(),
        }

        return self.error_token("Unexpected character.");
    }
}
