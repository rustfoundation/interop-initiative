# Type Layout — Makefile Example

This example shows how to call a Rust function from C using a plain `Makefile`,
without any special interop tooling.

It relates to the [type layout problem statement](../../problem-space/0003-type-layout.md):
both the Rust and C sides must agree on the memory layout of the shared struct,
otherwise field reads will be incorrect or undefined behaviour will occur.

## What it demonstrates

- A `#[repr(C)]` Rust struct whose layout is stable across the FFI boundary
- A Rust `extern "C"` function that returns the struct by value
- A matching C struct definition that reads the fields correctly
- A `Makefile` that builds the Rust static library with `cargo` and then
  compiles and links the C program with `gcc` — no CMake or other tooling required

## Requirements

- Rust (stable) and `cargo`
- `gcc`
- `make`

## Build and run
```bash
make
```

Expected output:
```
byte:    42
integer: 1000
flag:    true
```

## Why this matters

Most C/C++ projects already have a `Makefile`. This example shows the minimal
steps needed to add a Rust component to such a project:

1. Build the Rust code as a `staticlib` using `cargo build --release`
2. Link the resulting `.a` file into the C binary using `gcc -L... -l...`
3. Pass the required native linker flags using `rustc --print=native-static-libs`

The key constraint is that the Rust struct must use `#[repr(C)]` — without it,
Rust is free to reorder or pad fields differently from C, causing silent data corruption.
