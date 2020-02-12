use crate::chunk::{Chunk, OpCode};
use crate::scanner::{Scanner, Token, TokenType};
use crate::value::Value;

use std::mem;

#[derive(Default)]
struct Parser<'a> {
    current: Token<'a>,
    previous: Token<'a>,
    had_error: bool,
    panic_mode: bool,
}

enum Precedence {
    None,
    Assignment, // =
    Of,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}

pub struct Compiler<'a> {
    parser: Parser<'a>,
    compiling_chunk: Chunk,
    scanner: Scanner<'a>,
}

impl<'a> Compiler<'a> {
    pub fn new() -> Self {
        Compiler {
            parser: Parser::default(),
            compiling_chunk: Chunk::new(),
            scanner: Scanner::new(""),
        }
    }

    pub fn compile(&mut self, source: &'a str) -> Option<Chunk> {
        self.scanner = Scanner::new(source);

        self.parser.had_error = false;
        self.parser.panic_mode = false;

        self.advance();
        self.expression();
        self.consume(TokenType::EOF, "Expect end of expression.");
        self.end_compiler();
        if self.parser.had_error {
            None
        } else {
            let chunk = mem::replace(&mut self.compiling_chunk, Chunk::new());
            Some(chunk)
        }
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        &mut self.compiling_chunk
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
        self.error_at(&self.parser.previous.clone(), message);
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(&self.parser.current.clone(), message);
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

    fn emit_byte(&mut self, byte: OpCode) {
        let line = self.parser.previous.line;
        let chunk = self.current_chunk();
        chunk.write_chunk(byte, line);
    }

    fn emit_bytes(&mut self, byte1: OpCode, byte2: OpCode) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::OpReturn);
    }

    fn make_constant(&mut self, value: Value) -> usize {
        // Note: The original version tests for constant index > UINT8_MAX. Here constant index is
        // already a usize so we can just return it.
        self.current_chunk().add_constant(value)
    }

    fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(value);
        self.emit_byte(OpCode::OpConstant(constant))
    }

    fn end_compiler(&mut self) {
        self.emit_return()
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn number(&mut self) {
        let value: f64 = self.parser.previous.src.parse().unwrap();
        self.emit_constant(value)
    }

    fn unary(&mut self) {
        let operator_type = self.parser.previous.typ;

        // Compile the operand.
        self.parse_precedence(Precedence::Unary);

        // Emit the operator instruction.
        match operator_type {
            TokenType::Minus => self.emit_byte(OpCode::OpNegate),
            _ => unreachable!(),
        }
    }

    fn parse_precedence(&self, precedence: Precedence) {
        unimplemented!()
    }

    fn expression(&self) {
        self.parse_precedence(Precedence::Assignment);
    }
}
