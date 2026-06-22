#!/bin/bash
set -e

echo "=== Building Rust library ==="

cd rust
cargo build
cd ..

echo "=== Compiling C++ and linking ==="

# Explicit path to static library (more reliable for CI)
g++ cpp/main.cpp \
    rust/target/debug/libincompatible_allocators.a \
    -o example

echo "=== Running example ==="
./example