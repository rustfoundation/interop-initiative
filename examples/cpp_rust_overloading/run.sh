#!/usr/bin/env bash
# ^ support bash that's not in the standard `/bin` path

# A script that builds and runs the cpp_rust_overloading example

# bash "strict mode": fail the script if any command fails, or any variable is unset when used
set -euo pipefail

# If the script is run outside the example directory, change to the expected location
cd "$(dirname "$0")"

echo "Building and running the cpp_rust_overloading example..."
cargo run
