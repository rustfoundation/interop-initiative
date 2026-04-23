#!/usr/bin/env bash
set -e

echo "Running Rust example..."
cargo run

echo "Running C++ example..."
g++ main.cpp -o main
./main

