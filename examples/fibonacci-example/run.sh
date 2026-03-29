#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"

cd rust-library && cargo build
cd ..

g++ -o fibonacci_demo main.cpp rust-library/target/debug/libfibonacci.a -lpthread -ldl

./fibonacci_demo
