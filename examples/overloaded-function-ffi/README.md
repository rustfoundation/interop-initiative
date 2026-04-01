# Calling Overloaded C++ Functions from Rust

This example shows how to call overloaded C++ functions from Rust using wrapper functions.

## Overview

This example demonstrates a limitation when calling overloaded C++ functions from Rust using FFI (Foreign Function Interface).

In C++, function overloading allows multiple functions to share the same name with different parameter types. However, Rust does not support function overloading in FFI, as it requires unique symbol names for linking. Because of this, overloaded C++ functions cannot be called directly from Rust.

## What it demonstrates

- Two C++ overloads of `add()`, one for integers and one for doubles
- Wrapper functions (`add_int`, `add_double`) that give each overload its own name
- How Rust declares and calls those wrapper functions using `extern "C"`
- Why the calls need to be wrapped in `unsafe`

## Why this matters

Many C++ libraries rely heavily on overloaded functions. When calling such code from Rust, it is not possible to directly reference overloaded functions through FFI.

Rust supports generics and traits, which can provide behavior similar to function overloading. However, these abstractions do not work across FFI boundaries, because FFI requires concrete function names and fixed signatures.

To work around this limitation, developers must create wrapper functions with unique names. These wrappers allow Rust to correctly link and call the intended function.

## Requirements

- Rust (stable) with `cargo`
- A C++ compiler (`g++` or `clang++`)

## Build and run

From this directory:

```bash
./run.sh

Expected output:
```
Calling overloaded C++ functions from Rust..
add_int(2, 3) = 5
add_double(2.5, 3.5) = 6
```