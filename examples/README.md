# Examples

## Calling Overloaded C++ from Rust

Rust's FFI only speaks C, so you can't call overloaded C++ functions directly. The workaround: wrap each overload in a uniquely-named `extern "C"` function.

- **main.cpp** — Two overloaded `greet()` functions, wrapped as `greet_number` and `greet_name` with `extern "C"`.
- **main.h** — Header for the wrappers.
- **src/main.rs** — Calls both wrappers from Rust.
- **build.rs** — Compiles the C++ via the `cc` crate.
- **Cargo.toml** — Just needs `cc` as a build dep.

Run with `cargo run`.
