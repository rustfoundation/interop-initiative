use std::ffi::{c_double, c_int};

// These symbols are implemented in the C++ file and exposed through C ABI wrappers.
unsafe extern "C" {
    fn double_value_i32(value: c_int) -> c_int;
    fn double_value_f64(value: c_double) -> c_double;
}

/// Calls the wrapper that forwards to the C++ `int` overload.
pub fn call_overload_i32(value: c_int) -> c_int {
    // SAFETY: The symbol is defined in cpp/overload.cpp and linked by build.rs.
    unsafe { double_value_i32(value) }
}

/// Calls the wrapper that forwards to the C++ `double` overload.
pub fn call_overload_f64(value: c_double) -> c_double {
    // as explained in call_overload_i32
    unsafe { double_value_f64(value) }
}


