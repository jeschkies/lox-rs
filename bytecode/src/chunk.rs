use crate::memory::reallocate;

use std::mem;
use std::ptr;

pub enum OpCode {
    OpReturn,
}

pub struct Chunk {
    count: usize,
    capacity: usize,
    code: *mut u8,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            count: 0,
            capacity: 0,
            code: ptr::null_mut(),
        }
    }

    pub fn write_chunk(&mut self, byte: u8) {
        if self.capacity < self.count + 1 {
            let old_capacity = self.capacity;
            self.capacity = self.grow_capacity(old_capacity);
            self.code = self.grow_array(self.code, old_capacity, self.capacity);
        }

        unsafe { *self.code.offset(self.count as isize) = byte; }
        self.count += 1;
    }

    fn grow_capacity(&self, capacity: usize) -> usize {
        if capacity < 8 {
            8
        } else {
            capacity * 2
        }
    }

    fn grow_array(&self, previous: *mut u8, old_count: usize, count: usize) -> *mut u8 {
        reallocate(
            previous,
            mem::size_of::<u8>() * old_count,
            mem::size_of::<u8>() * count,
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
