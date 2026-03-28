use std::ffi::{c_uint, c_ulonglong};

// Fibonacci function exposed to C++
// Returns the nth Fibonacci number
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
