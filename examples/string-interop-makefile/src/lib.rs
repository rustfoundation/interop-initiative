use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Takes a C string from C, prints it in Rust, and returns a new C string.
///
/// # Safety
/// - `input` must be a valid, non-null, null-terminated C string.
/// - The caller must free the returned pointer using `free_string`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn process_string(input: *const c_char) -> *mut c_char {
    // SAFETY: caller guarantees input is a valid null-terminated C string
    let c_str = unsafe { CStr::from_ptr(input) };

    let rust_str = c_str.to_str().unwrap_or("(invalid utf-8)");
    println!("[Rust] received: \"{}\"", rust_str);

    let response = format!("Hello from Rust! You sent: {}", rust_str);
    CString::new(response)
        .expect("CString::new failed")
        .into_raw()
}

/// Frees a string that was allocated by Rust.
///
/// # Safety
/// - `ptr` must have been returned by `process_string`.
/// - Must not be called more than once for the same pointer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_string(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    // SAFETY: ptr was created by CString::into_raw in process_string
    unsafe {
        drop(CString::from_raw(ptr));
    }
}
