use std::fmt;

use crate::token::{Token, TokenType};

pub fn error(line: i32, message: &str) {
    report(line, "", message);
}

pub fn report(line: i32, where_: &str, message: &str) {
    eprintln!("[line {}] Error{}: {}", line, where_, message);
    // had_error = true; TODO: Use custom Error type
}

pub fn parser_error(token: &Token, message: &str) {
    if token.tpe == TokenType::EOF {
        report(token.line, " at end", message);
    } else {
        report(token.line, &format!(" at '{}'", token.lexeme), message);
    }
}

#[derive(Debug)]
pub enum Error {
    Parser,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Parser => write!(f, "ParserError"),
        }
    }
}
