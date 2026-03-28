#!/bin/bash
set -e

# Stop script on any error (important for CI)

echo "=== Building Rust library ==="

cd rust
cargo build
cd ..

echo "=== Compiling C++ and linking ==="

# Directly link static library (robust for CI)
g++ cpp/main.cpp \
    rust/target/debug/libpanic_across_ffi.a \
    -o example

echo "=== Running example ==="

./example