use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[unsafe(no_mangle)]
pub extern "C" fn ask_name() {
    println!("Rust: What is your name?");
}

/// # Safety
/// `name` must be a valid null-terminated C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn process_name(name: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(name) };
    let name_str = c_str.to_str().unwrap();

    let response = format!("Welcome, {}!", name_str);
    CString::new(response).unwrap().into_raw()
}

/// # Safety
/// `ptr` must have been returned by `process_name` and not already freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_string(ptr: *mut c_char) {
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}
