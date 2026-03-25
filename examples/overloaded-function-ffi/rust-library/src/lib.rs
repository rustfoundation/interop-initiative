// Declare external C++ wrapper functions
// These must be marked unsafe because they are foreign functions
unsafe extern "C" {
    fn add_int(a: i32, b: i32) -> i32;
    fn add_double(a: f64, b: f64) -> f64;
}

// Export Rust function that calls the C++ int wrapper
// no_mangle ensures the function name is preserved for linking
#[unsafe(no_mangle)]
pub extern "C" fn call_add_int(a: i32, b: i32) -> i32 {
    // Calling external functions is unsafe
    unsafe { add_int(a, b) }
}

// Export Rust function that calls the C++ double wrapper
#[unsafe(no_mangle)]
pub extern "C" fn call_add_double(a: f64, b: f64) -> f64 {
    unsafe { add_double(a, b) }
}
