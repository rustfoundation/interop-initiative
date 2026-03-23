# Calling Overloaded C++ Functions from Rust

This example shows a practical workaround for calling overloaded C++ functions from Rust.

Rust has no concept of overloading and as such, can not do C++ overload resolution at the FFI boundary. The example solves that by keeping the C++ overloads as it is and then, adding
very small `extern "C"` wrappers with unique names. This provides the illusion to Rust that they are different functions which we can then work with.

In this case:

- `double_value_i32` forwards to `double_value(int)`
- `double_value_f64` forwards to `double_value(double)`

This keeps the overload logic in C++, while giving Rust explicit symbols to call.

## What it demonstrates

- A C++ overload set with two functions sharing one name.
- A C ABI wrapper layer that selects the intended overload.
- Rust calling those wrappers through FFI and exposing clear, safe helper APIs.
- Building Rust and C++ together using Cargo and a simple `build.rs` script.

## Requirements

- Rust (stable) and `cargo`
- A C++ compiler (`clang++`, `g++`)

## Build and run

From the repository root:

```bash
cargo run --manifest-path examples/overloaded-cpp-from-rust/Cargo.toml
```

Expected output:

```text
double_value(21) [int overload] -> 42
double_value(21) [double overload] -> 42
```

## Why this matters

Overloaded APIs are common in C++. Without a naming bridge, Rust FFI calls to
those APIs result to abnormal behaviours. This example demonstrates a small,
readable strategy that works today: use explicit C ABI wrappers to choose the
overload, then keep Rust-side calls clear and explicit.

## Related

- [Call an overloaded C++ function from Rust (#14)](https://github.com/rustfoundation/interop-initiative/issues/14)
