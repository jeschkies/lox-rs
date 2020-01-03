mod chunk;
mod debug;
mod memory;
mod value;
mod vm;

use chunk::{Chunk, OpCode};
use debug::disassemble_chunk;
use vm::VM;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(1.2);
    chunk.write_chunk(OpCode::OpConstant(constant), 123);

    chunk.write_chunk(OpCode::OpReturn, 123);

    let mut vm = VM::new(&chunk);

    disassemble_chunk(&chunk, "test chunk");
    vm.interpret();

    // No need to free chunk since we implemented `Drop`.
    Ok(())
}
