
- Problem Name: rust_cpp_basic_interop
- Start Date: 21-02-2026
- Problem Statement PR: https://github.com/rustfoundation/interop-initiative/pull/30

## Summary
[summary]: #summary

Rust and C++ use different build system and calling conventions, So its difficult to integrate Rust code into existing C/C++ projects. In C++ it uses CMake or Make and others and for the Rust it uses Cargo.The difference in calling conventions and Application Binary Interfaces (ABI) between C++ and Rust is solved by creating a stable C-compatible interface (FFI) and use a compatible ABI in this cases ' C ABI' and keep functioon public between the two languages.

### C++ to Rust 

In this example, the C++ program calls a Rust function and passes input values to it.The values are sent from C++ to Rust through the function arguments. Rust receives these values and processes them inside the function.Since the data comes from outside Rust, it is handled carefully to ensure it is valid before being used.

### Rust to C++

In this example the goal is to allow a C++ program to call functions written in Rust.Since both languages follow different rules, Rust functions must be exposed using a C-compatible interface (`extern "C"` and `no_mangle`) so that C++ can recognize and call them correctly.The Rust code is compiled into a static library using Cargo. On the C++ side, the same function is declared with a matching signature, and the library is linked during compilation.The main challenge is coordinating the build process, because Rust and C++ use different tools, so the Rust library must be built first and then linked with the C++ program.

### Example Code
[example-code]: #example-code

```rust
 [`src/lib.rs`](https://github.com/shashu8660/interop-initiative/blob/add-example1/examples/rust-cpp-basic-interop/src/lib.rs)
```

```cpp
[`main.cpp`](https://github.com/shashu8660/interop-initiative/blob/add-example1/examples/rust-cpp-basic-interop/main.cpp)
```

### Note:

What can break the code and why this problem exists :

Rust and C++ handel data , memory , and safety in different ways, which makes interoperability challenging .
- C++ allows platform-dependent types like `long` etc 
- Rust use fixed type like `i64`.
- Rust focuses on safety , while C++ allows more flexibility , which can lead to errors if not handled carefully 

for example :

- `long` in c++
- Linux/macos -> 8 bytes
- windows -> 4 bytes

- `i64` in Rust 
- always 8 bytes

## Key Concepts

- `#[repr(C)]` ensures memory layout compatibility, Always use this for shared structs
- `extern "C"` enables cross-language function calls
- `#[no_mangle]` prevents name mangling
- `unsafe` is required for raw pointer handling in FFI
- Use fixed-width types (`int64_t`, `i64`)
- Avoid platform-dependent types like `long`


### How to Build and Run

Make sure you have Rust (Cargo) and a C++ compiler (clang++ or g++) installed.

1. Build the Rust library and C++ program:
   ```
   make
   ```

2. Run the executable:
   ```
   ./main
   ```

This will compile the Rust code into a static library and link it with the C++ program, then execute it.

## Example Output

./main
```
Message: System started
Time: 1001
Check: true
Message: Low memory warning
Time: 1002
Check: false
Message: Critical failure!
Time: 1003
Check: true
```



## Features
- Rust-based logging system (`info`, `warning`, `error`)
- C++ application calling Rust functions
- Static linking using Cargo and C++ compiler
- Build integration using Makefile
- Optional integration using CMake

## Build System Interoperability Challenges

- Cargo assumes control over compilation and linking
- C/C++ build systems also manage linking
- Coordination between them requires manual steps

## How It Works

1. Rust is compiled into a static library using Cargo
2. The static library is linked into a C++ program
3. The C++ program calls Rust functions using FFI
4. Makefile or CMake coordinates the build process

### What this example demonstrates 

This example showa how to safely pass struct between C++ to Rust.

The example handles:

- Using `#[repr(C)]` to quarentee layout compatibilty 
- Using fixed -width types (`int64_t` & `i64`)
- Passing raw pointer across FFI
- safely reading data in Rust using `unsafe`

## Limitations

This example highlights several current limitations:

- Manual integration between Cargo and C++ build systems
- No unified dependency management , Rust uses Cargo and C++ uses make or cmake 
- Platform-specific linking differences (macOS, Linux, Windows)
- Unsafe code is required for pointer handling