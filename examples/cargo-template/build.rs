use cpp_build;

/// Build the C++ code into a library that Rust can use
fn main() {
    // Cargo hides build script output, unless you put `cargo::warning=` in front of it
    println!("cargo::warning=Rust code ran while building the cargo-template example");
    cpp_build::build("src/main.rs");
}
