use crate::chunk::{Chunk, OpCode};
use crate::compiler::Compiler;
use crate::debug::disassemble_instruction;
use crate::value::{print_value, Value};

macro_rules! runtime_error {
    ( $vm:ident, $format:expr) => {{
        eprintln!($format);

        let instruction = $vm.ip as usize - $vm.chunk.code as usize;
        let line = $vm.chunk.lines[instruction];
        eprintln!("[line {:>4}] in script", line);

        $vm.reset_stack();
    }};
    ( $vm:ident, $format:expr, $( $arg:expr),* ) => {{
        eprintln!($format, $( $arg ),*);

        let instruction = $vm.ip as usize - $vm.chunk.code as usize;
        let line = $vm.chunk.lines[instruction];
        eprintln!("[line {:>4}] in script", line);

        $vm.reset_stack();
    }}
}

macro_rules! binary_op{
    ( $vm:ident, $value_constructor:expr, $op:tt ) => {
        {
            if !$vm.peek(0).is_number() || !$vm.peek(1).is_number() {
                runtime_error!($vm, "Operands must be numbers.");
                return InterpretResult::RuntimeError;
            }

            let b = $vm.pop().as_number();
            let a = $vm.pop().as_number();
            $vm.stack.push($value_constructor(a $op b));
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
                OpCode::OpNil => self.stack.push(Value::new_nil()),
                OpCode::OpTrue => self.stack.push(Value::new_bool(true)),
                OpCode::OpFalse => self.stack.push(Value::new_bool(false)),
                OpCode::OpEqual => {
                    let b = self.pop();
                    let a = self.pop();
                    self.stack.push(Value::new_bool(a == b));
                }
                OpCode::OpGreater => binary_op!(self, Value::new_bool, >),
                OpCode::OpLess => binary_op!(self, Value::new_bool, <),
                OpCode::OpAdd => binary_op!(self, Value::new_number, +),
                OpCode::OpSubtract => binary_op!(self, Value::new_number, -),
                OpCode::OpMultiply => binary_op!(self, Value::new_number, *),
                OpCode::OpDivide => binary_op!(self, Value::new_number, /),
                OpCode::OpNot => {
                    let value = self.stack.pop();
                    self.stack.push(Value::new_bool(self.is_falsey(value)))
                }
                OpCode::OpNegate => {
                    if !self.peek(0).is_number() {
                        runtime_error!(self, "Operand must be a number.");
                        return InterpretResult::RuntimeError;
                    }

                    let value = self.pop();
                    self.stack.push(Value::new_number(-value.as_number()));
                }
                OpCode::OpReturn => {
                    print_value(&self.pop());
                    println!();
                    return InterpretResult::Ok;
                }
            }
        }
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().expect("The stack is empty!")
    }

    fn reset_stack(&mut self) {
        // Set top of stack to the beginning
        self.stack.clear();
    }

    fn peek(&self, distance: usize) -> &Value {
        &self.stack[self.stack.len() - distance - 1]
    }

    fn is_falsey(&self, maybe_value: Option<Value>) -> bool {
        if let Some(value) = maybe_value {
            value.is_nil() || (value.is_bool() && !value.as_bool())
        } else {
            false
        }
    }

    fn read_constant(&self, index: usize) -> Value {
        self.chunk.constants[index].clone()
    }
}
