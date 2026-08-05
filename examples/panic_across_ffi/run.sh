#!/bin/bash
set -e

echo "=== Building Rust library ==="
cd rust
cargo build
cd ..

echo "=== Compiling C++ and linking ==="
g++ cpp/main.cpp \
    rust/target/debug/libpanic_across_ffi.a \
    -o example

echo "=== Running example ==="

# Run the program, but DO NOT fail CI if it aborts
if ./example; then
    echo "Panic over FFI should abort, but it didn't"
    exit 1
fi
