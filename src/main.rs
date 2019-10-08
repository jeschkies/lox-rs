mod scanner;
mod token;

use std::io::{self, BufRead};
use std::process::exit;
use std::{env, fs};

use scanner::Scanner;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
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

fn error(line: i32, message: String) {
    report(line, "".to_string(), message);
}

fn report(line: i32, where_: String, message: String) {
    eprintln!("[line {}] Error{}: {}", line, where_, message);
    // had_error = true; TODO: Use custom Error type
}
