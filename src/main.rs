use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.as_slice() {
        [_, file] => runFile(file),
        [_] => runPrompt(),
        _ => {
            eprintln!("Usage: lox-rs [script]");
            exit(64)
        }
    }
}

fn runFile(path: &str) {
    println!("Run file {}", path)
}

fn runPrompt() {
    println!("run prompt")
}
