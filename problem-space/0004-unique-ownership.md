- Problem Name: unique_ownership
- Start Date: 2026-02-19
- Problem Statement PR: [rustfoundation/interop-initiative#1](https://github.com/rustfoundation/interop-initiative/pull/1)
- Rust Issue: [rust-lang/rust#0000](https://github.com/rust-lang/rust/issues/0000)
- C++ Issue: [????/????#0000](https://github.com/????/????/pull/0000)

## Summary
[summary]: #summary

Rust and C++ have different unique ownership concepts.
In Rust, unique ownership is enforced by the compiler on most non-reference types, but there are multiple ways to opt-out (including `Copy` and various `unsafe` APIs).
In C++, ownership can be managed using `unique_ptr`, but significant amounts of code handle ownership implicitly.

Passing a C++ `unique_ptr` to Rust, or obtaining one from an owned Rust value, needs to handle the semantic and API differences between the languages.
For example, Zngur does this using `Ref` and `RefMut` types.

Some (all?) FFI tools limit C++ types on the Rust stack, due to destructive moves. Fixing this might require language changes.
Some users might benefit from custom move operation support in Rust.

## Related Problems
[related-problems]: #related-problems

Ownership is related to:

- shared and exclusive references
- [correct memory allocation and deallocation](0001-incompatible-allocators.md)
- [compatible type layouts](0003-type-layout.md), or marshalling data using FFI glue

TODO: fill in the remainder of this section

## Impact
[impact]: #impact

TODO

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
