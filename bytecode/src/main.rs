mod chunk;
mod compiler;
mod debug;
mod error;
mod memory;
mod scanner;
mod value;
mod vm;

use std::fs;
use std::io::{self, Read};
use std::process::exit;

use debug::disassemble_chunk;
use error::Error;
use vm::{InterpretResult, VM};

struct Lox {
    vm: VM,
}

impl Lox {
    fn new() -> Self {
        Lox { vm: VM::new() }
    }

    fn repl(&mut self) -> Result<(), Error> {
        let mut line = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        loop {
            print!("> ");

            if handle.read_to_string(&mut line)? == 0 {
                println!();
                break;
            }

            self.vm.interpret(&line);
        }

        Ok(())
    }

    fn run_file(&mut self, path: &str) -> Result<(), Error> {
        let source = format!("{}\0", fs::read_to_string(path)?);

        match self.vm.interpret(&source) {
            InterpretResult::CompileError => exit(65),
            InterpretResult::RuntimeError => exit(70),
            InterpretResult::Ok => Ok(()),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    //let mut chunk = Chunk::new();

    let mut program = Lox::new();

    let args: Vec<String> = std::env::args().collect();
    match args.as_slice() {
        [_, file] => program.run_file(file)?,
        [_] => program.repl()?,
        _ => {
            eprintln!("Usage: lox-rs [script]");
            exit(64)
        }
    }
    //disassemble_chunk(&chunk, "test chunk");

    // No need to free chunk since we implemented `Drop`.
    Ok(())
}
