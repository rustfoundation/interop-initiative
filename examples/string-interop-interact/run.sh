

#!/usr/bin/env bash
# This script builds and runs the string interop example (C++ ↔ Rust)

# Exit immediately if a command fails, treat unset variables as errors
set -euo pipefail

# Use default C++ compiler if not provided
if [ -z "${CXX:-}" ]; then
  echo "CXX not set, using default compiler: c++"
  CXX=c++
fi

# Ensure script runs from its own directory
cd "$(dirname "$0")"

echo "Building Rust library..."

# Step 1: Build Rust static library
cd rust-lib
cargo build --release
cd ..

echo "Compiling C++ code..."

# Step 2: Compile C++ and link Rust static library
$CXX main.cpp rust-lib/target/release/librust_lib.a -o main

echo "Running program..."

# Step 3: Run executable
./main