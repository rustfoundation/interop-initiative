- Problem Name: incompatible_allocators
- Start Date: 2026-02-18
- Problem Statement PR: [rustfoundation/interop-initiative#1](https://github.com/rustfoundation/interop-initiative/pull/1)
- Rust Issue: [rust-lang/rust#0000](https://github.com/rust-lang/rust/issues/0000)
- C++ Issue: [????/????#0000](https://github.com/????/????/pull/0000)

## Summary
[summary]: #summary

Rust and C++ code can be compiled with different memory allocators (or allocation settings).
Allocating memory with one allocator, but deallocating it with an incompatible allocator, causes unsoundness.

This happens when ownership is passed from Rust to C++, then the memory is deallocated using C++ deallocation routines.
(Or the other way around.) Deallocation can happen explicitly using `drop`/`delete`, or implicitly at the end of a block.

FFI safety documentation often focuses on ensuring the same layout in Rust and C++. But deallocation also needs to be compatible for soundness.

## Related Problems
[related-problems]: #related-problems

This problem is related to:

- `drop`/destructor mismatch, which can be a code correctness or a soundness issue
- [type layout](0003-type-layout.md): some allocator data structures use the type system

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
