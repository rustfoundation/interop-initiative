//! A Rust library that provides math functions callable from C++

/// Adds two integers and returns the result
///
/// # Safety
///
/// The entire binary must only have one function with this un-mangled name.
/// Callers must use the C calling convention to call this function.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Multiplies two integers and returns the result
///
/// # Safety
///
/// The entire binary must only have one function with this un-mangled name.
/// Callers must use the C calling convention to call this function.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(unsafe { add(3, 4) }, 7);
        assert_eq!(unsafe { add(-1, 1) }, 0);
        assert_eq!(unsafe { add(0, 0) }, 0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(unsafe { multiply(3, 4) }, 12);
        assert_eq!(unsafe { multiply(-2, 5) }, -10);
        assert_eq!(unsafe { multiply(0, 100) }, 0);
    }
}
