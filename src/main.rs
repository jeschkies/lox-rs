mod error;
mod interpreter;
mod parser;
mod scanner;
mod syntax;
mod token;

use std::io::{self, BufRead};
use std::process::exit;
use std::{env, fs};

use error::Error;
use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;
use syntax::AstPrinter;

struct Lox {
    interpreter: Interpreter,
}

impl Lox {
    fn new() -> Self {
        Lox {
            interpreter: Interpreter,
        }
    }

    fn run_file(&self, path: &str) -> Result<(), Error> {
        let source = fs::read_to_string(path)?;
        self.run(source)
    }

    fn run_prompt(&self) -> Result<(), Error> {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            self.run(line?); // Ignore error.
            print!("> ");
        }
        Ok(())
    }

    fn run(&self, source: String) -> Result<(), Error> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        let mut parser = Parser::new(tokens);
        let statements = parser.parse()?;
        self.interpreter.interpret(&statements)?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let args: Vec<String> = env::args().collect();
    let lox = Lox::new();
    match args.as_slice() {
        [_, file] => match lox.run_file(file) {
            Ok(_) => (),
            Err(Error::Runtime { .. }) => exit(70),
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
