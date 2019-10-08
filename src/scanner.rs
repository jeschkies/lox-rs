use crate::token::{Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: i32,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            "null".to_string(),
            self.line,
        ));
        &self.tokens
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' => {
                if self.r#match('=') {
                    self.add_token(TokenType::BangEqual, None)
                } else {
                    self.add_token(TokenType::Bang, None)
                }
            }
            '=' => {
                if self.r#match('=') {
                    self.add_token(TokenType::EqualEqual, None)
                } else {
                    self.add_token(TokenType::Equal, None)
                }
            }
            '<' => {
                if self.r#match('=') {
                    self.add_token(TokenType::LessEqual, None)
                } else {
                    self.add_token(TokenType::Less, None)
                }
            }
            '>' => {
                if self.r#match('=') {
                    self.add_token(TokenType::GreaterEqual, None)
                } else {
                    self.add_token(TokenType::Greater, None)
                }
            }
            '/' => {
                if self.r#match('/') {
                    // A comment goes until the end of the line.
                    while (self.peek() != '\n' && !self.is_at_end()) {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None)
                }
            }
            _ => error(self.line, "Unexpected character."),
        }
    }

    fn r#match(&mut self, expected: char) -> bool {
        if (self.is_at_end()) {
            return false;
        }
        // TODO: !self.source.get(self.current..self.current).contains(expected)
        if (self
            .source
            .chars()
            .nth(self.current)
            .expect("Unexpected end of source.")
            != expected)
        {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source
                .chars()
                .nth(self.current)
                .expect("Unexpected end of source.")
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        // TODO: work on &str directly.
        let char_vec: Vec<char> = self.source.chars().collect();
        char_vec[self.current - 1]
    }

    fn add_token(&mut self, tpe: TokenType, literal: Option<String>) {
        let text = self
            .source
            .get(self.start..self.current)
            .expect("Source token is empty.");
        self.tokens.push(Token::new(
            tpe,
            text.to_string(),
            literal.unwrap_or("null".to_string()),
            self.line,
        ))
    }
}

// TODO: remove duplicate of reporting
fn error(line: i32, message: &str) {
    report(line, "".to_string(), message);
}

fn report(line: i32, where_: String, message: &str) {
    eprintln!("[line {}] Error{}: {}", line, where_, message);
    // had_error = true; TODO: Use custom Error type
}
