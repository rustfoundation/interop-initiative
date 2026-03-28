# panic_across_ffi

## Summary

This example demonstrates undefined behavior when a Rust panic crosses an FFI boundary into C++.

It shows both:
-  An unsafe case where a panic occurs in an `extern "C"` function
-  A safe approach where the panic is contained within Rust

---

## Problem

Rust panics use stack unwinding. However, unwinding across an FFI boundary (e.g., into C++ code) is **undefined behavior**.

To prevent this, Rust **does not allow panics to unwind through `extern "C"` functions**. Instead, it aborts the program.

---

## Example Behavior

This example intentionally includes both cases:

###  Safe Case

- Rust uses `std::panic::catch_unwind`
- Panic is handled entirely within Rust
- Program continues execution normally

###  Unsafe Case

- Rust panics inside an `extern "C"` function
- Panic attempts to cross into C++
- Rust aborts the program to prevent undefined behavior

---

## What Happens Internally

When a panic occurs inside an `extern "C"` function:

- Rust cannot safely unwind into foreign (C/C++) code
- Instead of allowing undefined behavior, Rust **terminates the program**
- This is why the unsafe example results in an abort

---

### Expected Output 
=== Calling SAFE Rust function ===
Rust: about to panic (SAFE version)...
Rust: panic was caught, no undefined behavior occurred.

=== Calling UNSAFE Rust function ===
Rust: about to panic (UNSAFE)...
... program aborts ...

## How to Run

```bash
./run.sh