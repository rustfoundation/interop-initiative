use std::ffi::{c_double, c_int};

// These symbols are implemented in the C++ file and exposed through C ABI wrappers.
unsafe extern "C" {
    fn multiply_2_int(value: c_int) -> c_int;
    fn multiply_2_double(value: c_double) -> c_double;
}

/// Calls the wrapper that forwards to the C++ `int` overload.
pub fn call_overload_int(value: c_int) -> c_int {
    // SAFETY: The symbol is defined in cpp/overload.cpp and linked by build.rs.
    unsafe { multiply_2_int(value) }
}

/// Calls the wrapper that forwards to the C++ `double` overload.
pub fn call_overload_double(value: c_double) -> c_double {
    // as explained in call_overload_int
    unsafe { multiply_2_double(value) }
}
