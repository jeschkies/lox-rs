use crate::chunk::{Chunk, OpCode};
use crate::value::print_value;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    for (i, op_code) in chunk.into_iter().enumerate() {
        disassemble_instruction(chunk, op_code, i)
    }
}

fn constant_instruction(name: &str, chunk: &Chunk, constant_index: usize) {
    print!("{:<16} {:>4} '", name, constant_index);
    print_value(&chunk.constants[constant_index]);
    println!("'");
}

fn simple_instruction(name: &str) {
    println!("{}", name)
}

pub fn disassemble_instruction(chunk: &Chunk, op_code: &OpCode, i: usize) {
    // Note: The index is not really the offset if the op code has different sizes. For now all
    // op codes have the same size.
    print!("{:04} ", i);
    if i > 0 && chunk.lines[i] == chunk.lines[i - 1] {
        print!("   | ");
    } else {
        print!("{:>4} ", chunk.lines[i])
    }
    match op_code {
        OpCode::OpConstant(constant_index) => {
            constant_instruction("OP_CONSTANT", chunk, *constant_index)
        }
        OpCode::OpNil => simple_instruction("OP_NIL"),
        OpCode::OpTrue => simple_instruction("OP_TRUE"),
        OpCode::OpFalse => simple_instruction("OP_FALSE"),
        OpCode::OpEqual => simple_instruction("OP_EQUAL"),
        OpCode::OpGreater => simple_instruction("OP_GREATER"),
        OpCode::OpLess => simple_instruction("OP_LESS"),
        OpCode::OpAdd => simple_instruction("OP_ADD"),
        OpCode::OpSubtract => simple_instruction("OP_SUBTRACT"),
        OpCode::OpMultiply => simple_instruction("OP_MULTIPLY"),
        OpCode::OpDivide => simple_instruction("OP_DIVIDE"),
        OpCode::OpNot => simple_instruction("OP_NOT"),
        OpCode::OpNegate => simple_instruction("OP_NEGATE"),
        OpCode::OpReturn => simple_instruction("OP_RETURN"),
        _ => println!("Unknown opcode {:?}", op_code),
    }
}
