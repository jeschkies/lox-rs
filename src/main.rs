mod class;
mod env;
mod error;
mod function;
mod interpreter;
mod object;
mod parser;
mod resolver;
mod scanner;
mod syntax;
mod token;

use std::fs;
use std::io::{self, BufRead};
use std::process::exit;

use error::Error;
use interpreter::Interpreter;
use parser::Parser;
use resolver::Resolver;
use scanner::Scanner;
use syntax::AstPrinter;

struct Lox {
    interpreter: Interpreter,
}

impl Lox {
    fn new() -> Self {
        Lox {
            interpreter: Interpreter::new(),
        }
    }

    fn run_file(&mut self, path: &str) -> Result<(), Error> {
        let source = fs::read_to_string(path)?;
        self.run(source)
    }

    fn run_prompt(&mut self) -> Result<(), Error> {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            self.run(line?); // Ignore error.
            print!("> ");
        }
        Ok(())
    }

    fn run(&mut self, source: String) -> Result<(), Error> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        let mut parser = Parser::new(tokens);
        let statements = parser.parse()?;

        let mut resolver = Resolver::new(&mut self.interpreter);
        resolver.resolve_stmts(&statements);

        if resolver.had_error {
            return Ok(());
        }

        self.interpreter.interpret(&statements)?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let args: Vec<String> = std::env::args().collect();
    let mut lox = Lox::new();
    match args.as_slice() {
        [_, file] => match lox.run_file(file) {
            Ok(_) => (),
            Err(Error::Return { .. }) => unreachable!(),
            Err(Error::Runtime { message, .. }) => {
                eprintln!("Error: {}", message);
                exit(70)
            }
            Err(Error::Parse) => exit(65),
            Err(Error::Io(_)) => unimplemented!(),
        },
        [_] => lox.run_prompt()?,
        _ => {
            eprintln!("Usage: lox-rs [script]");
            exit(64)
        }
    }
    Ok(())
}
