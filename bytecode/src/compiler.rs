use crate::chunk::Chunk;
use crate::scanner::{Scanner, Token, TokenType};

struct Parser<'a> {
    current: Token<'a>,
    previous: Token<'a>,
    had_error: bool,
    panic_mode: bool,
}

pub struct Compiler<'a> {
    parser: Parser<'a>,
    scanner: Scanner<'a>,
}

impl<'a> Compiler<'a> {
    pub fn compile(&mut self, source: &'a str, chunk: &mut Chunk) -> bool {
        let mut scanner = Scanner::new(source);

        self.parser.had_error = false;
        self.parser.panic_mode = false;

        self.advance();
        self.expression();
        self.consume(TokenType::EOF, "Expect end of expression.");
        !self.parser.had_error
    }

    fn error_at(&mut self, token: &Token, message: &str) {
        if self.parser.panic_mode {
            return;
        }
        self.parser.panic_mode = true;

        eprint!("[line {}] Error", token.line);

        match token.typ {
            TokenType::EOF => eprint!(" at end"),
            TokenType::Error => (),
            _ => eprint!(" at '{}'", token.src),
        }

        eprintln!(": {}", message);
        self.parser.had_error = true;
    }

    fn error(&mut self, message: &str) {
        self.error_at(&self.parser.previous, message);
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(&self.parser.current, message);
    }

    fn advance(&mut self) {
        self.parser.previous = self.parser.current;

        loop {
            self.parser.current = self.scanner.scan_token();
            if self.parser.current.typ != TokenType::Error {
                break;
            }

            self.error_at_current(self.parser.current.src);
        }
    }

    fn consume(&mut self, typ: TokenType, message: &str) {
        if self.parser.current.typ == typ {
            self.advance();
            return;
        }

        self.error_at_current(message);
    }

    fn expression(&self) {
        unimplemented!()
    }
}
