use std::ffi::{CString, CStr}; // Import CString (for owned C strings) and CStr (for borrowed C strings)
use std::os::raw::c_char; // Import C-compatible char type

// Step 1: Rust prints prompt
#[unsafe(no_mangle)] // Prevent Rust from changing the function name (needed for C++ linkage)
pub extern "C" fn ask_name() { // Expose function using C ABI so C++ can call it
    println!("Rust: What is your name?"); // Print prompt from Rust side
}

// Step 2: Rust processes input from C++
#[unsafe(no_mangle)] // Keep function name stable for FFI
pub extern "C" fn process_name(name: *const c_char) -> *mut c_char { 
    // `name` is a raw pointer from C++ (const char*)

    let c_str = unsafe { CStr::from_ptr(name) }; 
    // Convert raw pointer into a safe Rust CStr reference (unsafe because Rust trusts C++)

    let name_str = c_str.to_str().unwrap(); 
    // Convert CStr to Rust &str (UTF-8 string slice)

    let response = format!("Welcome, {}!", name_str); 
    // Create a new Rust String with formatted message

    CString::new(response).unwrap().into_raw()
    // Convert Rust String → CString → raw pointer
    // `into_raw()` transfers ownership to C++ (Rust will NOT free it automatically)
}

// Step 3: Free memory
#[unsafe(no_mangle)] // Required for C++ to call this function
pub extern "C" fn free_string(ptr: *mut c_char) {
    unsafe {
       let _ = CString::from_raw(ptr);
        // Reconstruct CString from raw pointer
        // When this goes out of scope, Rust automatically frees the memory
    }
}