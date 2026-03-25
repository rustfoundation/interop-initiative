// Calling overloaded C++ functions from Rust using extern "C" wrappers.
// C++ allows function overloading (same name, different parameter types),
// but Rust doesn't support this. So we wrap each C++ overload with a
// uniquely named extern "C" function, and call those from Rust.
// telling rust about the c wrapper functions we made in main.cpp
unsafe extern "C" {
    fn greet_number(x: i32);
    fn greet_name(x: *const std::ffi::c_char);
}

// calls both overloads of the C++ greet() function via wrappers
fn main() {
    // gotta use unsafe here bc rust can't check ffi stuff
    unsafe { greet_number(4) };
    // c"..." gives us a c string literal, .as_ptr() turns it into a raw pointer
    unsafe {
        greet_name(c"Yash".as_ptr());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    // Test that both overloaded C++ greet functions can be called
    #[test]
    fn test_greet_overloads() {
        // call the int overload
        unsafe { greet_number(42) };
        // call the string overload
        unsafe {
            greet_name(c"test".as_ptr());
        }
        writeln!(
            &mut std::io::stderr(),
            "Both greet overloads called successfully",
        )
        .expect("failed to write to stderr");
    }
}
