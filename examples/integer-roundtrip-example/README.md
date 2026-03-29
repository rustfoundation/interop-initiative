# Lossless Integer Roundtrip Example

This example demonstrates lossless integer roundtrips between Rust and C++ using proper FFI types.

## What it does

A C++ program sends an unsigned integer to a Rust function, which converts it and returns it losslessly back to C++.

This example uses `u32::MAX` (4294967295) to show that even large values roundtrip correctly.
When converted to a signed integer, the same bits read as `-1`, but the original value is preserved in the roundtrip.

## Why this matters

Rust and C++ have different integer types that don't always match across platforms.
Using `std::ffi` types like `c_uint` guarantees the types match correctly regardless of platform.

## How to run
```bash
bash run.sh
```

## Expected output
```
C++ sending value: 4294967295
Rust received: 4294967295
Rust converted to signed: -1
C++ received back: 4294967295
Roundtrip was lossless!
```
