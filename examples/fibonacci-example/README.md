# Fibonacci Example: Calling Rust from C++

This example demonstrates how to call a Rust function from C++ using FFI (Foreign Function Interface).

## What it does

A Rust function calculates the Fibonacci sequence and is called from C++.

The Fibonacci sequence is a series of numbers where each number is the sum of the two preceding ones:
0, 1, 1, 2, 3, 5, 8, 13, 21, 34...

## Files

- `lib.rs` - Rust library containing the fibonacci function
- `main.cpp` - C++ program that calls the Rust function
- `Makefile` - builds and links both languages together

## Requirements

- Rust compiler (`rustc`)
- G++ compiler (`g++`)

## How to run
```bash
make run
```

## Expected output
```
fibonacci(0) = 0
fibonacci(1) = 1
fibonacci(2) = 1
fibonacci(3) = 2
...
fibonacci(10) = 55
```

## How it works

1. Rust compiles `lib.rs` into a static library
2. C++ compiles `main.cpp` and links with the Rust library
3. C++ calls the Rust fibonacci function directly
