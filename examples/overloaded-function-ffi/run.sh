#!/bin/bash
set -e

echo "Running overloaded function FFI example..."

# Step 1: Build the Rust static library
cd rust-library
cargo build
cd ..

# Step 2: Compile and link C++ code with the Rust library
g++ main.cpp -Lrust-library/target/debug -lrust_library -o main

# Step 3: Run the final executable
./main
