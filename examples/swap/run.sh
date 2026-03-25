#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")"

echo "Running swap string using Makefile"

make clean
make run
