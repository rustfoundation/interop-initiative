use cpp::cpp;

// Declare the C++ functions in Rust
unsafe extern "C" {
    fn print_cpp_message();
    #[cfg(test)]
    fn test_cpp_message();
}

// `cpp!` sometimes uses the cached extracted C++ file, even when the C++ code has been changed.
// To force a rebuild, use `cargo clean` then `cargo build`.
cpp! {{
    // Replace this with your C++ code
    #include <iostream>

    // Prints a run message to the terminal
    extern "C" void print_cpp_message() {
        std::cout << "C++ code ran while running the cargo-template example" << std::endl;
    }

    // Prints a test message to the terminal
    extern "C" void test_cpp_message() {
        std::cout << "C++ code ran while testing the cargo-template example" << std::endl;
    }
}}

/// Prints a run message to the terminal, from Rust and C++
fn main() {
    // Replace this with your Rust code
    println!("Rust code ran while running the cargo-template example");
    unsafe { print_cpp_message() };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    // Optional: Replace this with your Rust tests, if you need any
    //
    /// Test printing a Rust and C++ message to the terminal
    #[test]
    fn test_rust_message() {
        // Rust tests hide test output, here's one way to print it anyway
        writeln!(
            &mut std::io::stderr(),
            "Rust code ran while testing the cargo-template example",
        )
        .expect("failed to write to stderr");

        unsafe { test_cpp_message() };
    }
}
