use std::ffi::{CString, CStr};
use std::os::raw::c_char;

// Rust → C++
#[unsafe(no_mangle)]
pub extern "C" fn get_message() -> *mut c_char {
    let msg = CString::new("Hello from Rust!").unwrap();
    msg.into_raw() // transfer ownership to C++
}

// C++ → Rust
#[unsafe(no_mangle)]
pub extern "C" fn print_message(ptr: *const c_char) {
    let c_str = unsafe { CStr::from_ptr(ptr) };
    let str_slice = c_str.to_str().unwrap();
    println!("Rust received: {}", str_slice);
}

// Free memory
#[unsafe(no_mangle)]
pub extern "C" fn free_string(ptr: *mut c_char) {
    unsafe {
        CString::from_raw(ptr);
    }
}