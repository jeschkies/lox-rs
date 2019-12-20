mod chunk;
mod debug;
mod memory;

use chunk::{Chunk, OpCode};
use debug::disassemble_chunk;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let mut chunk = Chunk::new();
    chunk.write_chunk(OpCode::OpReturn);

    disassemble_chunk(&chunk, "test chunk");

    // No need to free chunk since we implemented `Drop`.
    Ok(())
}
