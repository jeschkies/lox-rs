use crate::token::{Token, TokenType, KEYWORDS};

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

        self.tokens.push(Token::new(TokenType::EOF, "", self.line));
        &self.tokens
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.r#match('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.r#match('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.r#match('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.r#match('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            '/' => {
                if self.r#match('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => (), // Ignore whitespace
            '\n' => self.line += 1,
            '"' => self.string(),
            c => {
                if c.is_digit(10) {
                    self.number()
                } else if c.is_alphabetic() || c == '_' {
                    self.identifier()
                } else {
                    error(self.line, "Unexpected character.")
                }
            }
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        // See if the identifier is a reserved word.
        let text = self
            .source
            .get(self.start..self.current)
            .expect("Unexpected end.");

        let tpe: TokenType = KEYWORDS.get(text).cloned().unwrap_or(TokenType::Identifier);
        self.add_token(tpe);
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            // Consumer the ".".
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let n: f64 = self
            .source
            .get(self.start..self.current)
            .expect("Unexpected end.")
            .parse()
            .expect("Scanned number could not be parsed.");
        self.add_token(TokenType::Number { literal: n })
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        // Unterminated string.
        if self.is_at_end() {
            error(self.line, "Unterminated string.");
        }

        // The closing ".
        self.advance();

        // Trim the surrounding quotes.
        let literal = self
            .source
            .get((self.start + 1)..(self.current - 1))
            .expect("Unexpected end.")
            .to_string();
        self.add_token(TokenType::String { literal });
    }

    fn r#match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        // TODO: !self.source.get(self.current..self.current).contains(expected)
        if self
            .source
            .chars()
            .nth(self.current)
            .expect("Unexpected end of source.")
            != expected
        {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        self.source.chars().nth(self.current + 1).unwrap_or('\0')
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

    fn add_token(&mut self, tpe: TokenType) {
        let text = self
            .source
            .get(self.start..self.current)
            .expect("Source token is empty.");
        self.tokens.push(Token::new(tpe, text, self.line))
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
