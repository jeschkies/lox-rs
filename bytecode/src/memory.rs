use std::alloc;
use std::mem;
use std::ptr;

/// Re-implements reallocation.
///
/// This is for learning purposes and as the books states:
///
/// > This reallocate() function is the single function we’ll use for all dynamic memory management
/// > in clox—allocating memory, freeing it, and changing the size of an existing allocation.
/// > Routing all of those operations through a single function will be important later when we add
/// > a garbage collector that needs to keep track of how much memory is in use.
///
pub fn reallocate(previous: *mut u8, old_size: usize, new_size: usize) -> *mut u8 {
    // TODO: make generic when required.
    // TODO: we might just want to use libc::free and libc::realloc.
    unsafe {
        let layout = alloc::Layout::from_size_align(old_size, mem::align_of::<u8>())
            .expect("Could not determine Layout for reallocation.");
        if new_size == 0 {
            alloc::dealloc(previous as *mut u8, layout);
            ptr::null_mut()
        } else {
            alloc::realloc(previous as *mut u8, layout, new_size)
        }
    }
}

