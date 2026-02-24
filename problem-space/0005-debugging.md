- Problem Name: debugging
- Start Date: 2026-02-20
- Problem Statement PR: [rustfoundation/interop-initiative#2](https://github.com/rustfoundation/interop-initiative/pull/2)
- Rust Issue: [rust-lang/rust#0000](https://github.com/rust-lang/rust/issues/0000)
- C++ Issue: [????/????#0000](https://github.com/????/????/pull/0000)

## Summary
[summary]: #summary

The quality of debug information in mixed Rust/C++ projects is low, leading to difficulty debugging some code.

Rust uses C++-style debug info to interact with some debuggers, but this has drawbacks where the language semantics are different.
Less debugging is required for pure Rust projects, but when mixing Rust with C++, it is used a lot more.

### Example Code
[example-code]: #example-code

TODO

## Related Problems
[related-problems]: #related-problems

Debugging requires:

- [accurate information about type layouts](0003-type-layout.md)

And is related to:

- compliance tooling support for mixed binaries
- code patching for mixed binaries
- compatibility with existing logging solutions

TODO: fill in the remainder of this section

## Impact
[impact]: #impact

The following impacts have been reported by users:

- It is difficult to run the Rust `Display` impl from `lldb` (from a user of `cxx` for FFI)
- There are highlighting discrepancies
- There is no first-class support for common types, e.g. break on an `Option<T>` doesn't work
- When in Rust debug mode, the representation of a C++ type is less helpful
- Rust provides low-quality `.pdb` output on Windows
- Debugging support for Rust async varies based on the executor
- Post-mortem debugging needs stack traces to show up, regardless of language
  - Rust standard library does not fall back to the symbol table if DWARF debug info is absent

TODO: fill in the remainder of this section

## Guide-level explanation
[guide-level-explanation]: #guide-level-explanation

TODO

## Reference-level explanation
[reference-level-explanation]: #reference-level-explanation

TODO

## Acceptance Criteria
[acceptance-criteria]: #acceptance-criteria

TODO

## Prior art
[prior-art]: #prior-art

TODO

## Further Background
[further-background]: #further-background

TODO

## Experts & Champions
[experts--champions]: #experts--champions

TODO

## Unresolved questions
[unresolved-questions]: #unresolved-questions

TODO
