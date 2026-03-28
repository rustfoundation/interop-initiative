// Tell Rust that this function exists in C++
unsafe extern "C" {
    fn multiply(a: i32, b: i32) -> i32;
}


// Expose this function to C++ using the C ABI.
// `no_mangle` is marked unsafe because it controls the symbol name at link time.
#[unsafe(no_mangle)]
pub extern "C" fn add(left: i32, right: i32) -> i32 {
    let result = unsafe { multiply(left, right) };

    left + right + result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 3);
        assert_eq!(result, 11);
    }
}
