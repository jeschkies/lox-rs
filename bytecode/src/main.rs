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

    let constant_2 = chunk.add_constant(3.4);
    chunk.write_chunk(OpCode::OpConstant(constant_2), 123);

    chunk.write_chunk(OpCode::OpAdd, 123);

    let constant_3 = chunk.add_constant(5.6);
    chunk.write_chunk(OpCode::OpConstant(constant_3), 123);

    chunk.write_chunk(OpCode::OpDivide, 123);
    chunk.write_chunk(OpCode::OpNegate, 123);

    chunk.write_chunk(OpCode::OpReturn, 123);

    let mut vm = VM::new(&chunk);

    disassemble_chunk(&chunk, "test chunk");
    vm.interpret();

    // No need to free chunk since we implemented `Drop`.
    Ok(())
}
