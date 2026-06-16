# Calling Overloaded C++ Functions from Rust

This example shows a practical workaround for calling overloaded C++ functions from Rust.

Rust has no first class support for overloading and as such, can not do C++ overload resolution at the FFI boundary. The example solves that by keeping the C++ overloads as it is and then, adding
very small `extern "C"` wrappers with unique names. This provides the illusion to Rust that they are functions with different names (technically, they have different names though, but from the FFI boundary) which we can then work with.

In this case:

- `multiply_2_int` forwards to `multiply_2(int)`
- `multiply_2_double` forwards to `multiply_2(double)`

This keeps the overload logic in C++, while giving Rust explicit symbols to call.

## What it demonstrates

- A C++ overload set with two functions sharing one name.
- A C ABI wrapper layer that selects the intended overload.
- Rust calling those wrappers through FFI and exposing clear, safe helper APIs.
- Building Rust and C++ together using the `cpp` crate and a simple `build.rs` script.

## Requirements

- Rust (stable) and `cargo`
- A C++ compiler (`clang++`, `g++`)

## Build and run

From the repository root:

```bash
cargo run --manifest-path examples/overloaded-multiply-by-2/Cargo.toml
```

Expected output:

```text
multiply_2(21) [int overload] -> 42
multiply_2(21.0) [double overload] -> 42.0
```

## Why this matters

Overloaded APIs are common in C++. Without a naming bridge, Rust FFI calls to
those APIs result to abnormal behaviours. This example demonstrates a small,
readable strategy that works today: use explicit C ABI wrappers to choose the
overload, then keep Rust-side calls clear and explicit.

## Related

- [Call an overloaded C++ function from Rust (#14)](https://github.com/rustfoundation/interop-initiative/issues/14)
