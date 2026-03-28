#!/usr/bin/env bash
set -euo pipefail

if [ -z "${CXX:-}" ]; then
  CXX=c++
fi

cd "$(dirname "$0")"

echo "Building string interop example..."

# Build Rust
cd rust-lib
cargo build --release
cd ..

# Compile C++
$CXX main.cpp rust-lib/target/release/librust_lib.a -o main

# Run
./main