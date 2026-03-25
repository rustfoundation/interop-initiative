fn main() {
    // Re-run this build script whenever the C++ source file changes.
    println!("cargo:rerun-if-changed=cpp/overload.cpp");

    // Compile the C++ file and link it into this Rust crate.
    cc::Build::new()
        // Tell `cc` to compile in C++ mode.
        .cpp(true)
        // Source file containing the overloads and C ABI wrappers.
        .file("cpp/overload.cpp")
        // Prefer a modern C++ standard when the compiler supports it.
        .flag_if_supported("-std=c++17")
        // Name of the produced static library.
        .compile("overload_cpp");
}
