# Calling Overloaded C++ Functions from Rust

This example shows how to call overloaded C++ functions from Rust using wrapper functions.

## The problem

In C++, you can have multiple functions with the same name as long as they take different types. This is called overloading. Rust doesn't support this when calling foreign functions, so we need to give each version a unique name that Rust can actually call.

## What it demonstrates

- Two C++ overloads of `add()`, one for integers and one for doubles
- Wrapper functions (`add_int`, `add_double`) that give each overload its own name
- How Rust declares and calls those wrapper functions using `extern "C"`
- Why the calls need to be wrapped in `unsafe`

## Requirements

- Rust (stable) and `cargo`
- A C++ compiler (`g++` or `clang++`)

## Build and run

```bash
cd examples/cpp-overloaded-functions
cargo run
```

Expected output:
```
add_int(3, 4) = 7
add_double(1.5, 2.5) = 4
```

## Why this matters

A lot of C++ libraries use overloaded functions. If you want to call them from Rust, you need some way to tell Rust which version you mean. This example shows one way to do that by creating simple wrappers with different names.
