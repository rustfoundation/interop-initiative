#!/usr/bin/env bash
# A script that builds and runs the rust-in-cpp-build example

# Fail the script if any command fails
set -euo pipefail

# Use default C++ compiler if not set
if [ -z "${CXX:-}" ]; then
  CXX=c++
fi

# Change to the example directory
cd "$(dirname "$0")"

# Build the Rust code into a static library
rustc --crate-type=staticlib lib.rs -o librust_functions.a

# Build and run the C++ program linked with the Rust library
$CXX main.cpp -L. -lrust_functions -o main
./main