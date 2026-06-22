# Incompatible Allocators — Makefile Example

This example shows how to correctly allocate memory in Rust and free it
from C using a plain `Makefile`.

It relates to the [incompatible allocators problem statement](../../problem-space/0001-incompatible-allocators.md):
memory allocated by Rust **must** be freed by Rust. Calling C's `free()`
on a Rust-allocated pointer is undefined behaviour.

## What it demonstrates

- Allocating a `Vec<u32>` in Rust and passing ownership to C via a raw pointer
- A dedicated `rust_free_array()` function that takes back ownership and
  drops the Vec using Rust's allocator
- Why `free(arr)` in C would be undefined behaviour in this case
- How to use `std::mem::forget` to transfer ownership across the FFI boundary

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
[C] received array from Rust:
  arr[0] = 0
  arr[1] = 1
  arr[2] = 2
  arr[3] = 3
  arr[4] = 4
[C] array freed correctly using rust_free_array
```

## Key safety rule

The C code calls `rust_free_array(arr, length)` instead of `free(arr)`.
This is essential because `rust_alloc_array` uses Rust's global allocator
via `Vec`. Freeing with C's `free()` would use a different allocator,
causing undefined behaviour as described in
[0001-incompatible-allocators.md](../../problem-space/0001-incompatible-allocators.md).
