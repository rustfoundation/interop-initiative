use std::os::raw::c_uint;

/// Allocates an array of `length` u32 values on the Rust heap,
/// fills them with sequential values, and returns a raw pointer.
///
/// # Safety
/// - The caller must free this pointer using `rust_free_array`.
/// - Never pass this pointer to C's `free()` — undefined behaviour.
#[unsafe(no_mangle)]
pub extern "C" fn rust_alloc_array(length: c_uint) -> *mut c_uint {
    let length = length as usize;
    let mut vec: Vec<u32> = (0..length as u32).collect();
    let ptr = vec.as_mut_ptr();
    std::mem::forget(vec); // give ownership to the caller
    ptr
}

/// Frees an array that was allocated by `rust_alloc_array`.
///
/// # Safety
/// - `ptr` must have been returned by `rust_alloc_array`.
/// - `length` must match the length passed to `rust_alloc_array`.
/// - Must not be called more than once for the same pointer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_free_array(ptr: *mut c_uint, length: c_uint) {
    if ptr.is_null() {
        return;
    }
    let length = length as usize;
    // SAFETY: ptr was created by Vec::as_mut_ptr with this length
    unsafe {
        let _ = Vec::from_raw_parts(ptr, length, length);
        // Vec drops here, freeing memory with Rust's allocator
    }
}
