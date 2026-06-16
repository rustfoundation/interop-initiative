#!/usr/bin/env bash
# ^ support bash that's not in the standard `/bin` path

# A script that builds and runs the type-layout-makefile example

# This file needs to be executable:
# - on Linux or macOS, use `chmod +x run.sh` then `git add run.sh`
# - on Windows, use `git add --chmod=+x run.sh` or `git update-index --chmod=+x run.sh`

# bash "strict mode": fail the script if any command fails, or any variable is unset when used
set -euo pipefail

# If the script is run outside the example directory, change to the expected location
cd "$(dirname "$0")"

make all
