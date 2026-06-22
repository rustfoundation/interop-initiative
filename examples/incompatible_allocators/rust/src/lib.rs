// This function is exposed to C++ via FFI.
//
// It allocates memory in Rust using Box (Rust's global allocator),
// then transfers ownership of that memory to C++.
//
// IMPORTANT:
// The memory MUST be deallocated using the SAME allocator (Rust's),
// otherwise this leads to undefined behavior.
#[no_mangle]
pub extern "C" fn pass_to_cpp() {
    // Allocate a fixed-size array on the heap using Rust's allocator
    let list = Box::new([1u32, 2, 3, 4, 5, 6, 7]);

    // Convert Box into a raw pointer.
    // This transfers ownership — Rust will NOT automatically free this memory anymore.
    let ptr = Box::into_raw(list);

    // SAFETY:
    // - We are passing a valid pointer and correct length
    // - We are assuming C++ will handle ownership correctly (this is the bug!)
    unsafe {
        take_ownership_in_cplusplus(ptr as *mut u32, 7);
    }
}

// Declaration of the C++ function.
// This tells Rust that the implementation exists on the C++ side.
extern "C" {
    fn take_ownership_in_cplusplus(pointer: *mut u32, length: usize);
}
