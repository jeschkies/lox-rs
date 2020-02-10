use crate::chunk::{Chunk, OpCode};
use crate::compiler::Compiler;
use crate::debug::disassemble_instruction;
use crate::value::{print_value, Value};

macro_rules! binary_op{
    ( $sel:ident, $op:tt ) => {
        {
            let b = $sel.stack.pop().expect("The stack was empty!");
            let a = $sel.stack.pop().expect("The stack was empty!");
            $sel.stack.push(a $op b);
        }
    };
}

static STACK_MAX: usize = 245;

pub struct VM {
    chunk: Chunk,
    ip: *const OpCode,
    stack: Vec<Value>,
}

// TODO: replace with Result<_, Error>
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

impl VM {
    pub fn new() -> Self {
        let chunk = Chunk::new();
        let ip = chunk.code;
        VM {
            chunk: chunk,
            ip: ip,
            stack: Vec::with_capacity(STACK_MAX),
        }
    }

    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        let mut compiler = Compiler::new();

        if let Some(chunk) = compiler.compile(source) {
            self.chunk = chunk;
            self.ip = self.chunk.code;

            let result = self.run();

            // TODO: free chunk
            result
        } else {
            return InterpretResult::CompileError;
        }
    }

    fn run(&mut self) -> InterpretResult {
        let mut position: usize = 0; // TODO: infer position from self.ip.
        loop {
            let instruction: OpCode = unsafe {
                let r = self.ip.read();
                self.ip = self.ip.add(1);
                r
            };

            if cfg!(feature = "debug_trace_execution") {
                print!("          ");
                for slot in &self.stack {
                    print!("[{:?}]", slot);
                }
                println!();
                disassemble_instruction(&self.chunk, &instruction, position);
                position += 1;
            }

            match instruction {
                OpCode::OpConstant(index) => {
                    let constant = self.read_constant(index);
                    self.stack.push(constant);
                }
                OpCode::OpAdd => binary_op!(self, +),
                OpCode::OpSubtract => binary_op!(self, -),
                OpCode::OpMultiply => binary_op!(self, *),
                OpCode::OpDivide => binary_op!(self, /),
                OpCode::OpNegate => {
                    let value = self.stack.pop().expect("The stack was empty!");
                    self.stack.push(-value);
                }
                OpCode::OpReturn => {
                    print_value(self.stack.pop().expect("The stack was empty!"));
                    println!();
                    return InterpretResult::Ok;
                }
            }
        }
    }

    fn read_constant(&self, index: usize) -> Value {
        self.chunk.constants[index]
    }
}
