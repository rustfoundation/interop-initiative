# Calling Rust from C: swap_string Example

This example shows how to swap a string using a Rust function called from C.

## How it works

- The Rust function is exposed using `extern "C"` and `#[no_mangle]`
- `unsafe` is required because Rust cannot guarantee that raw pointers from C are valid or safe.
- C declares the function and links against the Rust library
- A `Makefile` that builds the Rust static library with `cargo` and then
  compiles and links the C program with `clang`

## Build and run

```sh
make
make run
```
## Ouput:
```
Before Swap: Hello World
After Swap: World Hello
```
