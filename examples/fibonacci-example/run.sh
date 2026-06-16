#!/usr/bin/env bash
# A script that builds, tests, and runs the fibonacci-example

# bash "strict mode": fail the script if any command fails, or any variable is unset when used
set -euo pipefail

# Replace unset $CXX variable with the empty string
if [ -z "${CXX:-}" ]; then
  echo "CXX environment variable is empty or unset, using the default 'c++' compiler"
  CXX=c++
fi

# If the script is run outside the example directory, change to the expected location
cd "$(dirname "$0")"

# Build the Rust library
pushd rust-library
cargo build
popd

# Build and run the C++ and Rust binary
$CXX main.cpp rust-library/target/debug/libfibonacci.a -o main
./main
