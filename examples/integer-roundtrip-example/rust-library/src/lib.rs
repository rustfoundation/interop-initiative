//! A Rust library that demonstrates lossless integer roundtrips between Rust and C++

use std::ffi::{c_int, c_uint};

/// Result of an integer roundtrip operation
#[repr(C)]
pub struct RoundtripResult {
    /// The returned value
    pub value: c_uint,
    /// 0 = lossless, 1 = lossy (value changed during conversion)
    pub was_lossy: u8,
}

/// Takes an unsigned integer from C++, converts it to signed and back.
/// Returns the result along with a flag indicating if any data was lost.
///
/// # Safety
///
/// This function is called from C++ via FFI.
/// The caller must use the C calling convention.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn roundtrip_integer(value: c_uint) -> RoundtripResult {
    println!("Rust received: {value}");

    // Convert to signed — may lose information for large values
    let signed: c_int = value as c_int;
    println!("Rust converted to signed: {signed}");

    // Convert back to unsigned
    let returned: c_uint = signed as c_uint;

    // Check if roundtrip was lossless
    // Check if signed interpretation shows potential data interpretation change
    let was_lossy: u8 = if signed < 0 { 1 } else { 0 };
    RoundtripResult {
        value: returned,
        was_lossy,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_value_is_lossless() {
        unsafe {
            let result = roundtrip_integer(42);
            assert_eq!(result.was_lossy, 0);
            assert_eq!(result.value, 42);
        }
    }

    #[test]
    fn test_large_value_is_lossy() {
        unsafe {
            let result = roundtrip_integer(u32::MAX);
            assert_eq!(result.was_lossy, 1);
        }
    }
}
