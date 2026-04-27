# String Interop — Makefile Example

This example demonstrates how to pass strings between C and Rust across
the FFI boundary using a plain `Makefile`. It relates to the
[string interop problem statement](../../problem-space/0002-string-interop.md)
and touches on the
[incompatible allocators problem](../../problem-space/0001-incompatible-allocators.md).

---

## Overview

One of the most common FFI pitfalls is memory ownership — specifically,
who is responsible for freeing a string. If Rust allocates a string and
hands a raw pointer to C, **C must not call `free()` on it**. The memory
was allocated by Rust's allocator, and only Rust knows how to release it
correctly. Doing otherwise is undefined behaviour.

This example makes that rule concrete with a minimal, working C + Rust
program.

---

## What it demonstrates

- Passing a C string (`const char*`) into Rust using `CStr`
- Returning a Rust-allocated string to C using `CString::into_raw`
- Freeing a Rust-allocated string correctly using a dedicated `free_string` function
- Why you must **never** call C's `free()` on a pointer returned by Rust

---

## Requirements

- Rust (stable) and `cargo`
- `gcc`
- `make`

---

## Build and run

```bash
make

Expected output:
```
[C] sending: "Hello from C!"
[Rust] received: "Hello from C!"
[C] received: "Hello from Rust! You sent: Hello from C!"
[C] string freed successfully
```

## Key safety rule

The C code calls `free_string(response)` instead of `free(response)`.
This is essential: `process_string` returns a pointer created by
`CString::into_raw`, which uses Rust's allocator. Freeing it with C's
`free()` would be undefined behaviour, as described in
[0001-incompatible-allocators.md](../../problem-space/0001-incompatible-allocators.md).
