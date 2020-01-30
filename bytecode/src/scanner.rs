use std::fmt;

pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: i32,
}

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

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source: source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        // TODO: `nth` is O(n). We should be faster.
        self.source.chars().nth(self.current).unwrap() == '\0'
    }

    fn advance(&mut self) -> char {
        self.current += 1;

        // TODO: `nth` is O(n). We should be faster.
        self.source.chars().nth(self.current - 1).unwrap()
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
            '('=> return self.make_token(TokenType::LeftParen),
            ')'=> return self.make_token(TokenType::RightParen),
            '{'=> return self.make_token(TokenType::LeftBrace),
            '}'=> return self.make_token(TokenType::RightBrace),
            ';'=> return self.make_token(TokenType::Semicolon),
            ','=> return self.make_token(TokenType::Comma),
            '.'=> return self.make_token(TokenType::Dot),
            '-'=> return self.make_token(TokenType::Minus),
            '+'=> return self.make_token(TokenType::Plus),
            '/'=> return self.make_token(TokenType::Slash),
            '*'=> return self.make_token(TokenType::Star),
            _ => unimplemented!(),
        }

        return self.error_token("Unexpected character.");
    }
}
