use crate::memory::reallocate;
use crate::value::{Value, ValueArray};

use std::mem;
use std::ptr;

#[derive(Debug)]
pub enum OpCode {
    OpConstant(usize),
    OpReturn,
}

pub struct Chunk {
    count: usize,
    capacity: usize,

    pub lines: Vec<u64>,
    pub constants: ValueArray,

    // We use a raw pointer to [OpCode] instance of a `u8` to simplify the array handling. This will
    // use up more memory than the solution in the book. The book stores `OpReturn` as a single byte
    // and `OpConstant` with the index as two bytes. Rust's enum will use two bytes for both. As this
    // implementation is for learning purposes this solution should suffice. A safe Rust
    // implementation should use `Vec<OpCode>` instead.
    code: *mut OpCode,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            count: 0,
            capacity: 0,
            code: ptr::null_mut(),
            lines: Vec::with_capacity(8),
            constants: Vec::with_capacity(8),
        }
    }

    /// Write a chunk of code.
    pub fn write_chunk(&mut self, byte: OpCode, line: u64) {
        // Grow code array if required.
        if self.capacity < self.count + 1 {
            let old_capacity = self.capacity;
            self.capacity = self.grow_capacity(old_capacity);
            self.code = self.grow_array(self.code, old_capacity, self.capacity);
        }

        // Write chunk to code array.
        unsafe {
            let value: *const OpCode = &byte;
            ptr::copy_nonoverlapping(
                value,
                self.code.offset(self.count as isize),
                mem::size_of::<OpCode>(),
            );
        }
        self.count += 1;

        // Write line into separate array.
        self.lines.push(line);
    }

    /// Adds a constant and returns the index to the inserted value.
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    fn grow_capacity(&self, capacity: usize) -> usize {
        if capacity < 8 {
            8
        } else {
            capacity * 2
        }
    }

    fn grow_array(&self, previous: *mut OpCode, old_count: usize, count: usize) -> *mut OpCode {
        reallocate(
            previous,
            mem::size_of::<OpCode>() * old_count,
            mem::size_of::<OpCode>() * count,
        )
    }

    fn free_array(&mut self) {
        reallocate(self.code, mem::size_of::<u8>() * self.capacity, 0);
    }
}

impl Drop for Chunk {
    /// This is called `freeChunk` in the book.
    fn drop(&mut self) {
        self.free_array();

        self.count = 0;
        self.capacity = 0;
        self.code = ptr::null_mut();
    }
}

impl<'a> IntoIterator for &'a Chunk {
    type Item = &'a OpCode;
    type IntoIter = ChunkIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ChunkIter {
            chunk: self,
            offset: 0,
        }
    }
}

pub struct ChunkIter<'a> {
    chunk: &'a Chunk,
    offset: usize,
}

impl<'a> Iterator for ChunkIter<'a> {
    type Item = &'a OpCode;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset < self.chunk.count {
            let result: &OpCode;
            unsafe {
                result = &*self.chunk.code.offset(self.offset as isize);
            }
            self.offset += 1;
            Some(result)
        } else {
            None
        }
    }
}
