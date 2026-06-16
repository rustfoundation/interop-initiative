// This build script tells Cargo how to compile the C++ file.
// Cargo runs this automatically before compiling main.rs.

fn main() {
    // Use the cc crate to compile add.cpp as C++ code
    // and link it into the final Rust binary
    cc::Build::new()
        .cpp(true) // Treat the file as C++ (not C)
        .file("src/add.cpp") // Path to our C++ source file
        .compile("add"); // Name of the output library
}
