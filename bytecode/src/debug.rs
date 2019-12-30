use crate::chunk::{Chunk, OpCode};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    for (i, op_code) in chunk.into_iter().enumerate() {
        disassemble_instruction(op_code, i)
    }
}

fn disassemble_instruction(op_code: OpCode, i: usize) {
    // Note: The index is not really the offset if the op code has different sizes. For now all
    // op codes have the same size.
    print!("{:04} ", i);
    match op_code {
        OpCode::OpReturn => simple_instruction("OP_RETURN"),
        _ => println!("Unknown opcode {:?}", op_code)
    }
}

fn simple_instruction(name: &str) {
    println!("{}", name)
}
