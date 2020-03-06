use crate::chunk::{Chunk, OpCode};
use crate::debug::disassemble_chunk;
use crate::scanner::{Scanner, Token, TokenType};
use crate::value::Value;

use std::collections::HashMap;
use std::convert::From;
use std::mem;
use std::ops::Add;

macro_rules! parse_rule {
    ( $m:ident, $token_type:ident => $prefix:expr, $infix:expr, $precedence:ident) => {{
        $m.insert(
            TokenType::$token_type,
            ParseRule::new($prefix, $infix, Precedence::$precedence),
        );
    }};
}

#[derive(Default)]
struct Parser<'a> {
    current: Token<'a>,
    previous: Token<'a>,
    had_error: bool,
    panic_mode: bool,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
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

impl From<i32> for Precedence {
    fn from(i: i32) -> Self {
        match i {
            0 => Precedence::None,
            1 => Precedence::Assignment,
            2 => Precedence::Of,
            3 => Precedence::And,
            4 => Precedence::Equality,
            5 => Precedence::Comparison,
            6 => Precedence::Term,
            7 => Precedence::Factor,
            8 => Precedence::Unary,
            9 => Precedence::Call,
            10 => Precedence::Primary,
            _ => unreachable!(),
        }
    }
}

impl Add<i32> for &Precedence {
    type Output = Precedence;

    fn add(self, other: i32) -> Precedence {
        Precedence::from(self.clone() as i32 + other)
    }
}

type ParseFn<'r> = fn(&mut Compiler<'r>) -> ();

struct ParseRule<'r> {
    prefix: Option<ParseFn<'r>>,
    infix: Option<ParseFn<'r>>,
    precedence: Precedence,
}

impl<'a> ParseRule<'a> {
    fn new(
        prefix: Option<ParseFn<'a>>,
        infix: Option<ParseFn<'a>>,
        precedence: Precedence,
    ) -> Self {
        ParseRule {
            prefix,
            infix,
            precedence,
        }
    }
}

pub struct Compiler<'a> {
    parser: Parser<'a>,
    compiling_chunk: Chunk,
    scanner: Scanner<'a>,
    parse_rules: HashMap<TokenType, ParseRule<'a>>,
}

impl<'a> Compiler<'a> {
    #[rustfmt::skip::macros(parse_rule)]
    pub fn new() -> Self {
        let parse_rules: HashMap<TokenType, ParseRule> = {
            let mut m: HashMap<TokenType, ParseRule> = HashMap::new();
            parse_rule!(m, LeftParen    => Some(Compiler::grouping), None,                   None);
            parse_rule!(m, RightParen   => None,                     None,                   None);
            parse_rule!(m, LeftBrace    => None,                     None,                   None);
            parse_rule!(m, RightBrace   => None,                     None,                   None);
            parse_rule!(m, Comma        => None,                     None,                   None);
            parse_rule!(m, Dot          => None,                     None,                   None);
            parse_rule!(m, Minus        => Some(Compiler::unary),    Some(Compiler::binary), Term);
            parse_rule!(m, Plus         => None,                     Some(Compiler::binary), Term);
            parse_rule!(m, Semicolon    => None,                     None,                   None);
            parse_rule!(m, Slash        => None,                     Some(Compiler::binary), Factor);
            parse_rule!(m, Star         => None,                     Some(Compiler::binary), Factor);
            parse_rule!(m, Bang         => Some(Compiler::unary),    None,                   None);
            parse_rule!(m, BangEqual    => None,                     Some(Compiler::binary), Equality);
            parse_rule!(m, Equal        => None,                     None,                   None);
            parse_rule!(m, EqualEqual   => None,                     Some(Compiler::binary), Equality);
            parse_rule!(m, Greater      => None,                     Some(Compiler::binary), Comparison);
            parse_rule!(m, GreaterEqual => None,                     Some(Compiler::binary), Comparison);
            parse_rule!(m, Less         => None,                     Some(Compiler::binary), Comparison);
            parse_rule!(m, LessEqual    => None,                     Some(Compiler::binary), Comparison);
            parse_rule!(m, Identifier   => None,                     None,                   None);
            parse_rule!(m, String       => None,                     None,                   None);
            parse_rule!(m, Number       => Some(Compiler::number),   None,                   None);
            parse_rule!(m, And          => None,                     None,                   None);
            parse_rule!(m, Class        => None,                     None,                   None);
            parse_rule!(m, Else         => None,                     None,                   None);
            parse_rule!(m, False        => Some(Compiler::literal),  None,                   None);
            parse_rule!(m, For          => None,                     None,                   None);
            parse_rule!(m, Fun          => None,                     None,                   None);
            parse_rule!(m, If           => None,                     None,                   None);
            parse_rule!(m, Nil          => Some(Compiler::literal),  None,                   None);
            parse_rule!(m, Or           => None,                     None,                   None);
            parse_rule!(m, Print        => None,                     None,                   None);
            parse_rule!(m, Return       => None,                     None,                   None);
            parse_rule!(m, Super        => None,                     None,                   None);
            parse_rule!(m, This         => None,                     None,                   None);
            parse_rule!(m, True         => Some(Compiler::literal),  None,                   None);
            parse_rule!(m, Var          => None,                     None,                   None);
            parse_rule!(m, While        => None,                     None,                   None);
            parse_rule!(m, Error        => None,                     None,                   None);
            parse_rule!(m, EOF          => None,                     None,                   None);
            m
        };

        Compiler {
            parser: Parser::default(),
            compiling_chunk: Chunk::new(),
            scanner: Scanner::new(""),
            parse_rules: parse_rules,
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
        self.emit_return();

        if cfg!(feature = "debug_trace_execution") {
            if self.parser.had_error {
                disassemble_chunk(self.current_chunk(), "code");
            }
        }
    }

    fn binary(&mut self) {
        // Remember the operator.
        let operator_type = self.parser.previous.typ;

        // Compile the right operand.
        let rule_precedence = &self.get_rule(&operator_type).precedence + 1;
        self.parse_precedence(rule_precedence);

        // Emit the operator instruction.
        match operator_type {
            TokenType::BangEqual => self.emit_bytes(OpCode::OpEqual, OpCode::OpNot),
            TokenType::EqualEqual => self.emit_byte(OpCode::OpEqual),
            TokenType::Greater => self.emit_byte(OpCode::OpGreater),
            TokenType::GreaterEqual => self.emit_bytes(OpCode::OpLess, OpCode::OpNot),
            TokenType::Less => self.emit_byte(OpCode::OpLess),
            TokenType::LessEqual => self.emit_bytes(OpCode::OpGreater, OpCode::OpNot),
            TokenType::Plus => self.emit_byte(OpCode::OpAdd),
            TokenType::Minus => self.emit_byte(OpCode::OpSubtract),
            TokenType::Star => self.emit_byte(OpCode::OpMultiply),
            TokenType::Slash => self.emit_byte(OpCode::OpDivide),
            _ => unreachable!(),
        }
    }

    fn literal(&mut self) {
        match self.parser.previous.typ {
            TokenType::False => self.emit_byte(OpCode::OpFalse),
            TokenType::Nil => self.emit_byte(OpCode::OpNil),
            TokenType::True => self.emit_byte(OpCode::OpTrue),
            _ => unreachable!(),
        }
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn number(&mut self) {
        let value: f64 = self.parser.previous.src.parse().unwrap();
        self.emit_constant(Value::new_number(value))
    }

    fn unary(&mut self) {
        let operator_type = self.parser.previous.typ;

        // Compile the operand.
        self.parse_precedence(Precedence::Unary);

        // Emit the operator instruction.
        match operator_type {
            TokenType::Bang => self.emit_byte(OpCode::OpNot),
            TokenType::Minus => self.emit_byte(OpCode::OpNegate),
            _ => unreachable!(),
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        let prefix_rule = self.get_rule(&self.parser.previous.typ).prefix;
        if let Some(rule) = prefix_rule {
            rule(self);

            while precedence <= self.get_rule(&self.parser.current.typ).precedence {
                self.advance();
                let infix_rule = self
                    .get_rule(&self.parser.previous.typ)
                    .infix
                    .expect("No infix defined.");
                infix_rule(self);
            }
        } else {
            self.error("Expect expression.");
        }
    }

    fn get_rule(&self, typ: &TokenType) -> &ParseRule<'a> {
        &self.parse_rules[typ]
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }
}
