mod chunk;
mod debug;
mod memory;
mod value;

use chunk::{Chunk, OpCode};
use debug::disassemble_chunk;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write_chunk(OpCode::OpConstant(constant), 123);

    chunk.write_chunk(OpCode::OpReturn, 123);

    disassemble_chunk(&chunk, "test chunk");

    // No need to free chunk since we implemented `Drop`.
    Ok(())
}
