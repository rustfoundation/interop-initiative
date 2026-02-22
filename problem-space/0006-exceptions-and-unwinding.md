- Problem Name: exceptions_and_unwinding
- Start Date: 2026-02-20
- Problem Statement PR: [rustfoundation/interop-initiative#2](https://github.com/rustfoundation/interop-initiative/pull/2)
- Rust Issue: [rust-lang/rust#0000](https://github.com/rust-lang/rust/issues/0000)
- C++ Issue: [????/????#0000](https://github.com/????/????/pull/0000)

## Summary
[summary]: #summary

The semantics of C++ exceptions and Rust panic unwinding are different.

Different projects handle this in different ways:

- disable C++ exceptions, but enable Rust panic unwinding
- use C++ exceptions for error handling
- turn C++ exceptions into Rust `Result::Error`s, but Rust panics aren't turned into C++ exceptions
- conditionally turn C++ exceptions into aborts, depending on the context the code runs in

Making Rust panics into C++ exceptions generates too much complexity, even though some C++ ABIs (Itanium) support C++ catching foreign exceptions.

## Related Problems
[related-problems]: #related-problems

Related to:

- [accurate information about type layouts](0003-type-layout.md)

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
