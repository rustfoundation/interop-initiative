#!/usr/bin/env bash
# ^ support bash that's not in the standard `/bin` path

# A script that builds, tests, and runs the build-tool-template example

# This file needs to be executable:
# - on Linux or macOS, use `chmod +x run.sh` then `git add run.sh`
# - on Windows, use `git add --chmod=+x run.sh` or `git update-index --chmod=+x run.sh`

# bash "strict mode": fail the script if any command fails, or any variable is unset when used
set -euo pipefail

# Replace unset `$CXX` variable with the empty string
if [ -z "${CXX:-}" ]; then
  echo "CXX environment variable is empty or unset, using the default 'c++' compiler"
  CXX=c++
fi

# If the script is run outside the example directory, change to the expected location
cd "$(dirname "$0")"

# Replace the lines below with your build tool command
echo "Shell script ran while running the build-tool-template example"
# Build the Rust library
pushd rust-library
cargo build
popd
# Build and run the C++ and Rust binary
# This order is important, dependencies must be later than code that depends on them
$CXX main.cpp rust-library/target/debug/librust_library.a -o main
./main
