- Problem Name: rust_cpp_basic_interop
- Start Date: 21-02-2026
- Problem Statement PR:

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
use std::ffi::CStr; //Cstr is used to convert C-style string 'const char*' into rust string 
use std::os::raw::c_char; // same matching C-style 'char'

// exposing the the function to c++
#[unsafe(no_mangle)] // this line used keep the function name of the of both C and rust same . This is essential for FFI 
pub extern "C" fn log_message(msg: *const c_char) { 
/*pub: its for making function public so both can access the function
extern: This keyword tells the Cargo that the function using different calling convention other than its  ABI
C: its telling to use ABI of the C-programming */
    print_log("[INFO]", msg);
}

//expose another function to c++ (no name mangling + C ABI)
#[unsafe(no_mangle)]
pub extern "C" fn log_warning(msg: *const c_char) {
    print_log("[WARNING]", msg);
}

//expose another function to c++ (no name mangling + C ABI)
#[unsafe(no_mangle)]
pub extern "C" fn log_error(msg: *const c_char) {
    print_log("[ERROR]", msg);
}

// Internal helper function (not exposed to C++)
fn print_log(level: &str, msg: *const c_char) {
    let c_str = unsafe { CStr::from_ptr(msg) };//converting the raw C string pointer into Rust CStr
    let str_slice = c_str.to_str().unwrap();//Convert CStr into Rust string slice (&str)

    println!("{} {}", level, str_slice);//printing 
}
```

```cpp

#include <iostream>  // For std::cout

// Declare Rust functions so C++ knows about them
// extern "C" ensures the function names match Rust (no C++ name mangling)
extern "C" void log_message(const char* msg);
extern "C" void log_warning(const char* msg);
extern "C" void log_error(const char* msg);

int main() {
    // Call Rust function with a message
    log_message("System started");

    // Call Rust function with warning message
    log_warning("Low memory warning");

    // Call Rust function with error message
    log_error("Critical failure!");

    return 0;  // End of program
}

```
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
[INFO] System started
[WARNING] Low memory warning
[ERROR] Critical failure!


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

This example demonstrates basic Rust and C++ interoperability using FFI by passing C-style strings from C++ to Rust.

The example handles:

- Passing `const char*` strings from C++ into Rust functions
- Converting C-style strings into Rust string slices using `CStr`
- Calling multiple Rust functions from a C++ program through a shared interface

## Limitations

- Manual integration between Cargo and C++ build systems
- No unified dependency management
- Platform-specific linking differences (macOS, Linux, Windows)