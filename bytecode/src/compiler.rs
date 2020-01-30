use crate::scanner::{Scanner, Token, TokenType};

pub struct Compiler;

impl Compiler {
    pub fn compile(&self, source: &str) {
        let mut scanner = Scanner::new(source);
        let mut line: i32 = -1;
        loop {
            let token = scanner.scan_token();
            if token.line != line {
                print!("{:>4}", token.line);
                line = token.line;
            } else {
                print!("   | ");
            }
            print!("{:>2} '{}'", token.typ, token.src);

            if token.typ == TokenType::EOF {
                break;
            }
        }
    }
}
