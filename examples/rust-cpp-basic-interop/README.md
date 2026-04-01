
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
use std::ffi::CStr; // Convert C-style string to Rust string
use std::os::raw::c_char;

// C-compatible struct shared between C++ and Rust
#[repr(C)]
pub struct LogMessage {
    pub msg: *const c_char,
    pub time: i64,
    pub check: bool,
}

/// # Safety
/// The caller must ensure that `msg` is a valid null-terminated C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_message(msg: *const c_char) {
    unsafe { print_log("[INFO]", msg); }
}

/// # Safety
/// The caller must ensure that `msg` is a valid null-terminated C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_warning(msg: *const c_char) {
    unsafe { print_log("[WARNING]", msg); }
}

/// # Safety
/// The caller must ensure that `msg` is a valid null-terminated C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_error(msg: *const c_char) {
    unsafe { print_log("[ERROR]", msg); }
}

/// # Safety
/// The caller must ensure that `log` is a valid pointer to LogMessage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_struct(log: *const LogMessage) {
    if log.is_null() {
        return;
    }

    let log_ref = unsafe { &*log };
    let c_str = unsafe { CStr::from_ptr(log_ref.msg) };
    let message = c_str.to_str().unwrap();

    println!("Message: {}", message);
    println!("Time: {}", log_ref.time);
    println!("Check: {}", log_ref.check);
}

// Internal helper function
unsafe fn print_log(level: &str, msg: *const c_char) {
    let c_str = unsafe { CStr::from_ptr(msg) };
    let str_slice = c_str.to_str().unwrap();

    println!("{} {}", level, str_slice);
}
```

```cpp
#include <iostream>
#include <cstdint>  // for int64_t

// Struct must match Rust layout
struct LogMessage {
    const char* msg;
    int64_t time;
    bool check;
};

// Declare Rust functions
extern "C" void log_struct(const LogMessage* log);

int main() {
    LogMessage log1 = {"System started", 1001, true};
    LogMessage log2 = {"Low memory warning", 1002, false};
    LogMessage log3 = {"Critical failure!", 1003, true};

    log_struct(&log1);
    log_struct(&log2);
    log_struct(&log3);

    return 0;
}
```

## Key Concepts

- `#[repr(C)]` ensures memory layout compatibility
- `extern "C"` enables cross-language function calls
- `#[no_mangle]` prevents name mangling
- `unsafe` is required for raw pointer handling in FFI


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

### Acceptance Criteria

This example demonstrates Rust and C++ interoperability using FFI by passing structured data from C++ to Rust.

The example handles:

- Passing structured data (`LogMessage`) from C++ to Rust
- Ensuring memory layout compatibility using `#[repr(C)]`
- Using fixed-width integer types (`int64_t` ↔ `i64`) for cross-platform safety
- Handling raw pointers safely using `unsafe` and documented safety contracts

## Limitations

- Manual integration between Cargo and C++ build systems
- No unified dependency management
- Platform-specific linking differences (macOS, Linux, Windows)