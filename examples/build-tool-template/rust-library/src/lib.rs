//! A Rust library that prints a message to the terminal

/// Prints a Rust run message to the terminal
///
/// # Safety
///
/// The entire binary must only have one function with this un-mangled name.
/// Callers must use the C calling convention to call this function.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn print_rust_message() {
    // Replace this with your Rust code
    println!("Rust code ran while running the build-tool-template example");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    // Optional: Replace this with your Rust tests, if you need any
    //
    /// Test printing a Rust message to the terminal
    #[test]
    fn test_rust_message() {
        // Rust tests hide test output, here's one way to print it anyway
        writeln!(
            &mut std::io::stderr(),
            "Rust code ran while testing the build-tool-template example",
        )
        .expect("failed to write to stderr");

        unsafe { print_rust_message() };
    }
}
