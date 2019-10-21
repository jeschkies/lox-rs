mod error;
mod scanner;
mod syntax;
mod token;

use std::io::{self, BufRead};
use std::process::exit;
use std::{env, fs};

use scanner::Scanner;
use syntax::{AstPrinter, Expr};
use token::{Token, TokenType};

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let expression = Expr::Binary {
        left: Box::new(Expr::Unary {
            operator: Token::new(TokenType::Minus,"-", 1),
            right: Box::new(Expr::Literal {
                value: "123".to_string(),
            }),
        }),
        operator: Token::new(TokenType::Star, "*", 1),
        right: Box::new(Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value: "45.67".to_string(),
            }),
        }),
    };
    let printer = AstPrinter;
    println!("{}", printer.print(expression));
    let args: Vec<String> = env::args().collect();
    match args.as_slice() {
        [_, file] => run_file(file)?,
        [_] => run_prompt()?,
        _ => {
            eprintln!("Usage: lox-rs [script]");
            exit(64)
        }
    }
    Ok(())
}

fn run_file(path: &str) -> io::Result<()> {
    let source = fs::read_to_string(path)?;
    run(source)
}

fn run_prompt() -> io::Result<()> {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        run(line?); // Ignore error.
        print!("> ");
    }
    Ok(())
}

fn run(source: String) -> io::Result<()> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{}", token);
    }
    Ok(())
}
