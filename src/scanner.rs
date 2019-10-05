use crate::token::{Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: i32,
    current: i32,
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

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while (!is_at_end()) {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            "null".to_string(),
            self.line,
        ));
        self.tokens
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
