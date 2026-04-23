# C++ Overloading in Rust

This example demonstrates how to call overloaded C++ functions from Rust.

## The Problem
C++ allows multiple functions to have the same name (overloading) as long as their parameter types differ. Rust's Foreign Function Interface (FFI) is based on the C ABI, which does not support overloading and requires unique names for every function.

## The Solution
We use `extern "C"` wrapper functions in C++ to provide unique, stable names for each overload.

## Structure
- `src/lib.cpp`: Contains the original C++ overloads and the `extern "C"` wrappers.
- `src/main.rs`: The Rust code that declares and calls the wrappers.
- `build.rs`: Compiles the C++ code using the `cc` crate.

## Running the Example
```bash
cargo run
```
