use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Receives a C string, processes it, and returns a Rust-allocated string.
///
/// # Safety
/// - The caller must free the returned pointer using `free_string`.
/// - Never pass the returned pointer to C's `free()` — undefined behaviour.
#[no_mangle]
pub unsafe extern "C" fn process_string(input: *const c_char) -> *mut c_char {
    let input = CStr::from_ptr(input).to_string_lossy();
    println!("[Rust] received: \"{}\"", input);
    let response = format!("Hello from Rust! You sent: {}", input);
    CString::new(response).unwrap().into_raw()
}

/// Frees a string that was allocated by `process_string`.
///
/// # Safety
/// - `ptr` must have been returned by `process_string`.
/// - Must not be called more than once for the same pointer.
#[no_mangle]
pub unsafe extern "C" fn free_string(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    // SAFETY: ptr was created by CString::into_raw
    drop(CString::from_raw(ptr));
}
