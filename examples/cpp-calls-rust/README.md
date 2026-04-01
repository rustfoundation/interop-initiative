# C++ calls Rust example

This example shows how to call a Rust function from C++ using a Makefile.

## How it works

- Rust code is compiled into a static library using Cargo
- The Rust function is exposed using `extern "C"` and `#[no_mangle]`
- C++ declares the function and links against the Rust library
- The Makefile builds both parts and links them together

## Build and run

```sh
make run
```
