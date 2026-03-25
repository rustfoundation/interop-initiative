## Interop Example Code

This directory contains interop code examples for use cases and problem statements.

### How to Add a New Example

1. Add a new directory in `interop/examples`, with a useful name
2. Add a runnable example, choosing one of these methods based on the primary build tool:

    - If you are using a C/C++ build tool, put the build tool command in a `run.sh` script
    - If you are using `cargo`, CI will automatically run `cargo build`, `cargo test`, and `cargo run`

3. If any fixes are needed, follow these instructions, to make your PR easier to review.
   Add one commit for each CI check, in this order:

    1. build the Rust crates
    2. test the Rust crates (optional)
    3. run the Rust and C++ binary, after building and linking it
    4. `clippy` lints: to ignore a Rust lint, use `#[expect(lint, reason = "why this is ok")]`
    5. Rust `doc` checks
    6. `rustfmt` check

If you need help getting started, try copying and modifying the `build-tool-template` or `cargo-template`
directories.

### Out of Scope

These checks aren't included in CI, because they vary too much between compilers and platforms:

    - C++ tests
    - `c++ -Wall` (the equivalent of Rust `clippy`)
    - C++ doc lints
    - C++ code formatting
