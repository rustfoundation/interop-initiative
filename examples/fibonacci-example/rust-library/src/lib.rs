use std::ffi::{c_uint, c_ulonglong};

/// Result type for fibonacci, compatible with C++
#[repr(C)]
pub struct FibResult {
    /// The fibonacci value (only valid if overflow is 0)
    pub value: c_ulonglong,
    /// 0 = success, 1 = overflow (n was too large)
    pub overflow: u8,
}

/// Calculates the nth Fibonacci number.
///
/// # Safety
///
/// This function is called from C++ via FFI.
/// The caller must use the C calling convention.
/// Check the overflow field before using the value.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fibonacci(n: c_uint) -> FibResult {
    if n > 93 {
        return FibResult {
            value: 0,
            overflow: 1,
        };
    }
    match n {
        0 => FibResult {
            value: 0,
            overflow: 0,
        },
        1 => FibResult {
            value: 1,
            overflow: 0,
        },
        _ => {
            let mut a: c_ulonglong = 0;
            let mut b: c_ulonglong = 1;
            for _ in 2..=n {
                let temp = a + b;
                a = b;
                b = temp;
            }
            FibResult {
                value: b,
                overflow: 0,
            }
        }
    }
}
#[test]
fn test_boundary_value() {
    unsafe {
        let result = fibonacci(93);
        assert_eq!(result.overflow, 0);
        let result = fibonacci(94);
        assert_eq!(result.overflow, 1);
    }
}
