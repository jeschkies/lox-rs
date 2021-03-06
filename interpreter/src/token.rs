use std::fmt;
use std::hash::{Hash, Hasher};
extern crate phf;

#[derive(Debug, Clone, PartialEq)]
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

    // Literals. They are encoded in the enum themselves. Thus we do not need the `Object literal`
    // used in the book.
    Identifier,
    String { literal: String },
    Number { literal: f64 },

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

    EOF,
}

// Generated via phf_codegen until proc_macro_hygiene is stable.
include!(concat!(env!("OUT_DIR"), "/keywords.rs"));

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub tpe: TokenType,
    pub lexeme: String,
    pub line: i32,
}

impl Token {
    pub fn new(tpe: TokenType, lexeme: &str, line: i32) -> Self {
        Self {
            tpe,
            lexeme: lexeme.to_string(),
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.tpe {
            TokenType::String { literal } => write!(f, "String {:?} {:?}", self.lexeme, literal),
            TokenType::Number { literal } => write!(f, "Number {:?} {:?}", self.lexeme, literal),
            _ => write!(f, "{:?} {:?}", self.tpe, self.lexeme),
        }
    }
}

impl Hash for Token {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.lexeme.hash(state);
        self.line.hash(state);
    }
}

impl Eq for Token {}
