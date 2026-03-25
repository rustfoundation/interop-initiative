# Overloaded Greet

A small example showing how to call overloaded C++ functions from Rust.

## What it does

The C++ side has two overloads of `greet()` — one taking `int` and one taking `const char*`.
Since Rust doesn't support function overloading, each overload is wrapped with a unique name
(`greet_number`, `greet_name`) using `extern "C"`, and called from Rust via FFI.

## How to run

From the repository root:

```bash
cargo run --manifest-path examples/overloaded-greet/Cargo.toml
```

Expected output:

```
Number: 4
Hello: Yash
```
