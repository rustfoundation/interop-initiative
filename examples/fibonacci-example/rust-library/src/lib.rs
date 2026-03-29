use std::ffi::{c_uint, c_ulonglong};

/// Calculates the nth Fibonacci number.
///
/// # Safety
///
/// This function is called from C++ via FFI.
/// The caller must use the C calling convention and ensure
/// the function is only called with a valid unsigned integer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fibonacci(n: c_uint) -> c_ulonglong {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a: c_ulonglong = 0;
            let mut b: c_ulonglong = 1;
            for _ in 2..=n {
                let temp = a + b;
                a = b;
                b = temp;
            }
            b
        }
    }
}
