# Rust + C++ Interoperability Example

## Overview
This project demonstrates how to integrate Rust into an existing C++ project using a build system like Make or CMake.

It showcases calling Rust functions from C++ using FFI and linking them using a static library.

---

## Features
- Rust-based logging system (`info`, `warning`, `error`)
- C++ application calling Rust functions
- Static linking using Cargo and C++ compiler
- Build integration using Makefile
- Optional integration using CMake

---

## Real-World Use Case
This example simulates integrating a Rust-based logging system into an existing C++ application.

In real-world systems, developers may want to:
- Gradually introduce Rust into legacy C/C++ codebases
- Use Rust for safer components (e.g., logging, parsing)
- Retain existing build systems like Make or CMake

---

## Build System Interoperability Challenges

- Cargo assumes control over compilation and linking
- C/C++ build systems also manage linking
- Coordination between them requires manual steps

---

## How It Works

1. Rust is compiled into a static library using Cargo
2. The static library is linked into a C++ program
3. The C++ program calls Rust functions using FFI
4. Makefile or CMake coordinates the build process

---

## Limitations

- Manual integration between Cargo and C++ build systems
- No unified dependency management
- Platform-specific linking differences

---

## Example Output

./main
[INFO] System started
[WARNING] Low memory warning
[ERROR] Critical failure!
