use crate::chunk::{Chunk, OpCode};
use crate::debug::disassemble_instruction;
use crate::value::{print_value, Value};

pub struct VM<'a> {
    chunk: &'a Chunk,
    ip: *const OpCode,
}

// TODO: replace with Result<_, Error>
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

impl<'a> VM<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        VM {
            chunk: chunk,
            ip: chunk.code,
        }
    }

    pub fn interpret(mut self) -> InterpretResult {
        self.run()
    }

    fn run(mut self) -> InterpretResult {
        let mut position: usize = 0; // TODO: infer position from self.ip.
        loop {
            let instruction: OpCode = unsafe {
                let r = self.ip.read();
                self.ip = self.ip.add(1);
                r
            };

            if cfg!(feature = "debug_trace_execution") {
                disassemble_instruction(self.chunk, &instruction, position);
                position += 1;
            }

            match instruction {
                OpCode::OpConstant(index) => {
                    let constant = self.read_constant(index);
                    print_value(constant);
                    println!();
                }
                OpCode::OpReturn => return InterpretResult::Ok,
            }
        }
    }

    fn read_constant(&self, index: usize) -> Value {
        self.chunk.constants[index]
    }
}
