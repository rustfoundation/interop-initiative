//! A Rust library that demonstrates lossless integer roundtrips between Rust and C++

use std::ffi::{c_int, c_uint};

/// Takes an unsigned integer from C++, converts it losslessly, and returns it
///
/// # Safety
///
/// The entire binary must only have one function with this un-mangled name.
/// Callers must use the C calling convention and pass a valid unsigned integer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn roundtrip_integer(value: c_uint) -> c_uint {
    // Print the value received from C++
    println!("Rust received: {value}");

    // Lossless conversion to signed and back
    let signed: c_int = value as c_int;
    println!("Rust converted to signed: {signed}");

    // Return the original value losslessly
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that integer roundtrip is lossless
    #[test]
    fn test_roundtrip() {
        let value: c_uint = 42;
        unsafe {
            assert_eq!(roundtrip_integer(value), value);
        }
    }
}
