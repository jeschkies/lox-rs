use crate::chunk::{Chunk, OpCode};
use crate::debug::disassemble_instruction;
use crate::value::{print_value, Value};

static STACK_MAX: usize = 245;

pub struct VM<'a> {
    chunk: &'a Chunk,
    ip: *const OpCode,
    stack: Vec<Value>,
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
            stack: Vec::with_capacity(STACK_MAX),
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
                print!("          ");
                for slot in &self.stack {
                    print!("[{:?}]", slot);
                }
                println!();
                disassemble_instruction(self.chunk, &instruction, position);
                position += 1;
            }

            match instruction {
                OpCode::OpConstant(index) => {
                    let constant = self.read_constant(index);
                    self.stack.push(constant);
                }
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
