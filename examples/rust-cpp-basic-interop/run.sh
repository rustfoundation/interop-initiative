#!/usr/bin/env bash
# Use bash from system environment 

set -euo pipefail
# -e → exit if any command fails
# -u → error if using undefined variables
# -o pipefail → catch errors in piped commands

# Check if C++ compiler is set, otherwise use default
if [ -z "${CXX:-}" ]; then
  echo "CXX not set, using default compiler: c++"
  CXX=c++
fi

# Change directory to the location of this script
# This ensures the script works correctly in CI environments
cd "$(dirname "$0")"

echo "Building Rust + C++ interop example..."

# Build the Rust library using Cargo for my example 
# --release builds optimized static library
cargo build --release

# Compile the C++ program and link with Rust static library
# $CXX allows flexibility (clang++ or g++)
$CXX main.cpp target/release/librust_interop.a -o main

# Run the final executable
./main
