- Problem Name: type_layout
- Start Date: 2026-02-19
- Problem Statement PR: [rustfoundation/interop-initiative#1](https://github.com/rustfoundation/interop-initiative/pull/1)
- Rust Issue: [rust-lang/rust#0000](https://github.com/rust-lang/rust/issues/0000)
- C++ Issue: [????/????#0000](https://github.com/????/????/pull/0000)

## Summary
[summary]: #summary

Rust and C++ code can be compiled with different ABIs (compilers, compiler settings, and architectures).
Different ABIs can lead to different type layouts in memory, including size, alignment, field offsets, and padding.

Using the same layout in Rust and C++ enables zero-overhead data access, which is important for performance-sensitive use cases.
Mismatching layouts can lead to unsoundness, if data is not marshalled using the correct FFI glue code.

### Example Code
[example-code]: #example-code

TODO

## Related Problems
[related-problems]: #related-problems

This problem is related to:

- cross-language build coordination

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
